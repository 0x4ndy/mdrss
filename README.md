# mdrss
A Rust library for generating RSS feeds from markdown files.

## Usage
The main API function to generate an RSS feed from markdown files.
```rust
pub fn generate_rss(
    markdown_dir: &str,
    rss_output_path: &str,
    rss_conf: &RssConf,
) -> Result<()>
```
### Arguments
`markdown_dir` - A path to the directory containing the markdown files.
`rss_output_path` - The destination path for the generated RSS feed (rss.xml).
`rss_conf` - RSS configuration structure

## Example
[mdrss-cli](https://github.com/0x4ndy/mdrss-cli) is a CLI application that makes use of `mdrss` library.
