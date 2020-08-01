use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Article {
    rendered_body: String,
    body: String,
}

pub fn process_article_json_files() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir("data/articles")?;
    for entry in entries {
        let path = entry?.path();
        let article_json = fs::read_to_string(&path)?;

        let article_markdown = convert_article_json(&article_json)?;

        let mut output_path: PathBuf = [".", "output", "articles"].iter().collect();
        output_path.push(path.file_name().unwrap());
        output_path.set_extension("md");
        let mut file = File::create(output_path)?;
        file.write_all(article_markdown.as_bytes())?;
    };
    Ok(())
}

fn convert_article_json(article_json: &str) -> Result<String, Box<dyn std::error::Error>> {
    let article: Article = serde_json::from_str(&article_json)?;
    Ok(article.body)
}
