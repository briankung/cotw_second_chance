mod actions_summary;
mod link_count;
mod post;
mod twir;

use crate::post::Post;
use crate::twir::cotw_urls;

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
    let mut weeks = posts_by_week(&posts);
    let mut url_scores = HashMap::new();

    for week in weeks.iter_mut() {
        week.sort_by_cached_key(|p| -p.like_count());
        let week = week
            .iter()
            .filter(|post| post.like_count().is_positive() && !post.link_counts.is_empty());

        for post in week {
            if let Some(most_clicked) = post.link_counts.iter().max_by_key(|lc| lc.clicks) {
                let most_clicked_url = most_clicked.url.clone();
                let likes = post.like_count();

                url_scores
                    .entry(most_clicked_url.to_lowercase())
                    .and_modify(|s| *s += likes)
                    .or_insert(likes);
            }
        }
    }

    let mut sorted: Vec<_> = url_scores.into_iter().collect();
    sorted.sort_by_key(|(_, likes)| -likes);

    let cotw_urls = cotw_urls();

    println!("likes,url");

    for (url, likes) in sorted.iter().filter(|(url, _likes)| !cotw_urls.contains(url)) {
        println!("{},{}", likes, url);
    }

    Ok(())
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
