use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use pulldown_cmark::CowStr::Borrowed;
use pulldown_cmark::Event::End;
use pulldown_cmark::LinkType::Inline;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag::Link;

const COTW_HEADER: &str = "# crate";

fn twir_editions() -> impl Iterator<Item = PathBuf> {
    let dirs = fs::read_dir("this-week-in-rust/content/").expect("the directory");
    let paths = dirs.filter_map(Result::ok).map(|e| e.path());
    paths.filter_map(|path| {
        let file_name = path.file_name().and_then(OsStr::to_str);
        let extension = path.extension().and_then(OsStr::to_str);
        let mut result = None;

        if let (Some(fname), Some(ext)) = (file_name, extension) {
            if fname.contains("this-week") && (ext == "md" || ext == "markdown") {
                result = Some(path);
            };
        };

        result
    })
}

pub fn cotw_urls() -> HashSet<String> {
    let mut cotw_urls = HashSet::new();
    let header_length = COTW_HEADER.len();

    let cotw = twir_editions().filter_map(|path| {
        let content = fs::read_to_string(path).ok()?;
        let lowercase = content.to_lowercase();
        let start = lowercase.find(COTW_HEADER).map(|i| i + header_length)?;
        let length = lowercase[start..].find('#');
        let end = if let Some(length) = length {
            start + length
        } else {
            content.len()
        };

        lowercase
            .contains(COTW_HEADER)
            .then(|| content[start..end].to_owned())
    });

    for content in cotw {
        let parser = Parser::new(&content);
        for event in parser {
            if let End(Link(Inline, Borrowed(url), Borrowed(""))) = event {
                if !url.contains("/users") {
                    cotw_urls.insert(url.to_owned());
                }
            }
        }
    }

    cotw_urls
}
