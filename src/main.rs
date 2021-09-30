use askama::Template;
use glob::glob;
use std::fs;
use std::path::Path;

mod model;

use crate::model::{unfound_page, PageData, PageTemplate};

fn main() {
    // fs::create_dir_all("dist").unwrap();
    fs::create_dir_all("dist/content").unwrap_or_default();

    for entry in glob("content/*.md").unwrap() {
        if let Ok(path) = entry {
            fs::copy(&path, Path::new("dist").join(&path)).unwrap_or_default();

            let file_contents = fs::read_to_string(&path).unwrap();
            let file_parts: Vec<&str> = file_contents.splitn(3, "+++").collect();
            let data: PageData = toml::from_str(file_parts[1]).unwrap();
            let content: String = file_parts[2].to_string();
            let template = PageTemplate {
                data: data,
                content: content,
                is_xhr: false,
            };
            let file_stem = path.file_stem().unwrap();
            let new_file = format!("dist/{}.html", file_stem.to_str().unwrap());
            fs::write(new_file, template.render().unwrap()).unwrap();
        }
    }

    let mut template = unfound_page();

    template.is_xhr = false;

    fs::write("dist/404.html", template.render().unwrap()).unwrap();
}
