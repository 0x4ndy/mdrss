use mdrss::{generate_rss, RssConf};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_generate_rss() {
    // Create a temporary directory for markdown files
    let temp_dir = tempdir().unwrap();
    let markdown_dir = temp_dir.path().join("markdowns");
    fs::create_dir_all(&markdown_dir).unwrap();

    // Create a mock markdown file in the temp directory
    let md_file_path = markdown_dir.join("test.md");
    let content = r#"
-rss-
title: "Test Title"
pub_date: "2023-09-14T12:34:56Z"
author: "John Doe"
url: "http://example.com"
description: "A test description."
-rss-
"#;
    fs::write(&md_file_path, content).unwrap();

    // Path for the output RSS file
    let rss_output_path = temp_dir.path().join("rss.xml");

    let rss_conf = RssConf {
        title: String::from("Custom RSS Title"),
        link: String::from("https://example.com"),
        description: String::from("A test description."),
        delimiter: String::from("-rss-"),
    };

    // Call the API function to generate the RSS
    generate_rss(
        markdown_dir.to_str().unwrap(),
        rss_output_path.to_str().unwrap(),
        &rss_conf,
    )
    .expect("Failed to generate RSS feed");

    // Verify that the RSS file is created and contains expected content
    let rss_content = fs::read_to_string(rss_output_path).unwrap();
    assert!(rss_content.contains("<title>Test Title</title>"));
    assert!(rss_content.contains("<link>http://example.com</link>"));
    assert!(rss_content.contains("<description><![CDATA[A test description.]]></description>"));
}
