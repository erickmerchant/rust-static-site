use askama::Template;
use glob::glob;
use std::fs;

mod model;

use crate::model::{PageData,PageTemplate};

fn main() {
    for entry in glob("content/*.md").unwrap() {
        if let Ok(path) = entry {
            let file_contents = fs::read_to_string(&path).unwrap();

            let file_parts: Vec<&str> = file_contents.splitn(3, "+++").collect();

            let data: PageData = toml::from_str(file_parts[1]).unwrap();

            let content: String = file_parts[2].to_string();

            let template = PageTemplate {
                data: data,
                content: content,
            };

            fs::create_dir_all("dist").unwrap();

            let file_stem = path.file_stem().unwrap();

            let new_file = format!("dist/{}.html", file_stem.to_str().unwrap());

            fs::write(new_file, template.render().unwrap()).unwrap();
        }
    }
}
