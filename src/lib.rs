use chrono::{DateTime, Utc};
use rss::{ChannelBuilder, ItemBuilder};
use serde::Deserialize;
use std::fs::File;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

// Struct to hold the parsed front matter
#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String,
    pub_date: String,
    author: String,
    url: String,
    description: String,
}

// Function to parse the publication date as a `DateTime<Utc>`
fn parse_pub_date(date_str: &str) -> Result<DateTime<Utc>, chrono::format::ParseError> {
    date_str.parse::<DateTime<Utc>>()
}

// Function to parse front matter from a markdown file
fn parse_front_matter(content: &str, delimiter: &str) -> Option<FrontMatter> {
    let parts: Vec<&str> = content.splitn(3, delimiter).collect();
    if parts.len() == 3 {
        serde_yaml::from_str(parts[1]).ok()
    } else {
        None
    }
}

// Struct to hold an RSS item along with its parsed publication date
struct RssItem {
    pub_date: DateTime<Utc>,
    item: rss::Item,
}

// Function to process a markdown file and extract the RSS item information
fn process_markdown_file(path: &Path, delimiter: &str) -> Option<RssItem> {
    fs::read_to_string(path).ok().and_then(|content| {
        parse_front_matter(&content, delimiter).and_then(|front_matter| {
            parse_pub_date(&front_matter.pub_date).ok().map(|pub_date| {
                let item = ItemBuilder::default()
                    .title(Some(front_matter.title))
                    .pub_date(Some(front_matter.pub_date))
                    .author(Some(front_matter.author))
                    .link(Some(front_matter.url))
                    .description(Some(front_matter.description))
                    .build();

                RssItem { pub_date, item }
            })
        })
    })
}

// Function to traverse directories and process all markdown files
fn collect_markdown_files(dir: &Path, delimiter: &str) -> Vec<RssItem> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok()) // Handle invalid directory entries
        .filter(|entry| {
            entry.path().is_file()
                && entry.path().extension().and_then(|s| s.to_str()) == Some("md")
        })
        .filter_map(|entry| process_markdown_file(entry.path(), delimiter))
        .collect::<Vec<_>>() // Collect all valid markdown files
}

pub struct RssConf {
    pub title: String,
    pub link: String,
    pub description: String,
    pub delimiter: String,
}

/// The main API function to generate an RSS feed from markdown files.
///
/// # Arguments
///
/// * `markdown_dir` - A path to the directory containing the markdown files.
/// * `rss_output_path` - The destination path for the generated RSS feed (rss.xml).
/// * `rss_conf` - RSS configuration structure
///
pub fn generate_rss(
    markdown_dir: &str,
    rss_output_path: &str,
    rss_conf: &RssConf,
) -> io::Result<()> {
    // Convert strings to PathBuf
    let directory = PathBuf::from(markdown_dir);
    let output_path = PathBuf::from(rss_output_path);

    // Collect markdown files and generate RSS items
    let mut rss_items = collect_markdown_files(&directory, rss_conf.delimiter.as_str());

    // Sort items by publication date (descending)
    rss_items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

    // Build the RSS feed with sorted items
    let channel = ChannelBuilder::default()
        .title(rss_conf.title.as_str())
        .link(rss_conf.link.as_str())
        .description(rss_conf.description.as_str())
        .items(
            rss_items
                .into_iter()
                .map(|rss_item| rss_item.item)
                .collect::<Vec<_>>(),
        )
        .build();

    // Write the RSS feed to an XML file with pretty formatting
    let mut file = File::create(output_path)?;
    channel
        .pretty_write_to(&mut file, b' ', 2)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::fs;

    #[test]
    fn test_parse_pub_date() {
        let date_str = "2023-09-14T12:34:56Z";
        let parsed_date = parse_pub_date(date_str).unwrap();
        let expected_date = Utc.with_ymd_and_hms(2023, 9, 14, 12, 34, 56).unwrap();
        assert_eq!(parsed_date, expected_date);
    }

    #[test]
    fn test_parse_front_matter() {
        let content = r#"
-rss-
title: Test Title
pub_date: 2023-09-14T12:34:56Z
author: John Doe
url: http://example.com
description: A test description.
-rss-
"#;
        let front_matter = parse_front_matter(content, "-rss-").unwrap();
        assert_eq!(front_matter.title, "Test Title");
        assert_eq!(front_matter.pub_date, "2023-09-14T12:34:56Z");
        assert_eq!(front_matter.author, "John Doe");
        assert_eq!(front_matter.url, "http://example.com");
        assert_eq!(front_matter.description, "A test description.");
    }

    #[test]
    fn test_collect_markdown_files() {
        // Create a temp directory with mock markdown files
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.md");
        let content = r#"
-rss-
title: Test Title
pub_date: 2023-09-14T12:34:56Z
author: John Doe
url: http://example.com
description: A test description.
-rss-
"#;
        fs::write(&file_path, content).unwrap();

        // Collect markdown files
        let rss_items = collect_markdown_files(temp_dir.path(), "-rss-");
        assert_eq!(rss_items.len(), 1);
        assert_eq!(rss_items[0].item.title(), Some("Test Title"));
    }
}
