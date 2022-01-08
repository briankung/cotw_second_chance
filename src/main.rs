mod actions_summary;
mod base_url;
mod link_count;
mod post;
mod twir;

use crate::base_url::BaseUrl;
use crate::post::Post;
use crate::twir::cotw_sections;
use regex::Regex;

use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::{fs, path::Path};

type BoxedError = Box<dyn std::error::Error>;

const DATA_FILENAME: &str = "2704.json";

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    fetch_data().await?;

    let posts = parse_data()?;
    let sorted: Vec<(String, i64)> = extract_urls_likes(posts)
        .into_iter()
        .filter(|(url, _likes)| {
            url.contains("github.com")
                || url.contains("github.io")
                || url.contains("gitlab.com")
                || url.contains("crates.io")
                || url.contains("lib.rs")
                || url.contains("docs.rs")
        })
        .collect();

    // let cotw_urls = cotw_urls();

    let re = Regex::new(r#"[\w_-]+"#)?;

    let slugs: HashMap<String, &str> = sorted
        .iter()
        .map(|(url, _)| {
            let slug = re.find_iter(url).last().map(|c| c.as_str()).unwrap();
            (url.clone(), slug)
        })
        .filter(|(_url, slug)| slug.len() > 2)
        .collect();

    let cotw_content: String = cotw_sections().collect();

    println!("likes,url");

    sorted
        .par_iter()
        .filter(|(url, _)| {
            if let Some(slug) = slugs.get(url) {
                let query = &format!(r"(?i)((crate.+week)|(week.+crate)).+\b{}\b", slug)[..];
                let re = Regex::new(query).unwrap();
                !re.is_match(&cotw_content)
            } else {
                false
            }
        })
        .for_each(|(sorted_url, likes)| {
            println!("{},{}", likes, sorted_url);
        });

    Ok(())
}

// TODO: This is unnecessarily complicated due to (initially) grouping posts by week.
// Keep `posts_by_week` but don't use it in this function.
fn extract_urls_likes(posts: Vec<Post>) -> Vec<(String, i64)> {
    let mut url_scores = HashMap::new();
    let posts = posts
        .iter()
        .filter(|post| post.like_count().is_positive() && !post.link_counts.is_empty());

    for post in posts {
        let likes = post.like_count();

        for link_count in post.link_counts.iter() {
            let url = link_count.url.clone();
            let url = url.base_url();

            url_scores
                .entry(url.to_lowercase())
                .and_modify(|s| *s += likes)
                .or_insert(likes);
        }
    }

    url_scores.into_iter().collect()
}

async fn fetch_data() -> Result<(), BoxedError> {
    if Path::new(DATA_FILENAME).exists() && std::env::var("FORCE_DOWNLOAD").is_err() {
        return Ok(());
    }

    let target = "https://users.rust-lang.org/t/crate-of-the-week/2704.json?print=true";
    let response = reqwest::get(target).await?;
    let content = response.text().await?;
    let mut dest = File::create(DATA_FILENAME)?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}

fn parse_data() -> Result<Vec<Post>, BoxedError> {
    let data = fs::read_to_string(DATA_FILENAME)?;
    let json: serde_json::Value = serde_json::from_str(&data)?;
    let posts = json
        .get("post_stream")
        .and_then(|x| x.get("posts"))
        .expect("Unexpected json structure");
    let posts: Vec<Post> = serde_json::from_value(posts.clone())?;

    // Just making sure they're already sorted by created_at
    assert_sorted_by_created_at(&posts);

    Ok(posts)
}

fn assert_sorted_by_created_at(unsorted: &[Post]) {
    let mut sorted = unsorted.to_owned();
    sorted.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    assert_eq!(sorted[..], unsorted[..]);
}

fn posts_by_week(posts: &[Post]) -> Vec<Vec<Post>> {
    let mut weeks: Vec<Vec<Post>> = Vec::new();
    let mut week: Vec<Post> = Vec::new();
    let mut posts_iter = posts.iter();

    let first_post = posts_iter.next().unwrap();
    let first_post_timestamp = first_post.created_at.expect("post must have created_at");

    posts_iter.fold(first_post_timestamp, |prev_timestamp, post| {
        let post_timestamp = &post.created_at.expect("post must have created_at");

        if *post_timestamp - prev_timestamp >= time::Duration::WEEK {
            weeks.push(week.drain(..).collect());
            week.push(post.clone());
            *post_timestamp
        } else {
            week.push(post.clone());
            prev_timestamp
        }
    });

    weeks
}
