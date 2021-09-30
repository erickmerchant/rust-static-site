use askama::Template;
use serde::Deserialize;
use std::process;
use std::str::FromStr;
use toml::value::Datetime;

mod filters {
    use askama::Result;
    use chrono::NaiveDate;
    use pulldown_cmark::{html, Options, Parser};
    use toml::value::Datetime;
    pub fn markdown(s: &str) -> Result<String> {
        let mut content = String::new();
        let md_parser = Parser::new_ext(&s, Options::empty());
        html::push_html(&mut content, md_parser);
        Ok(content)
    }
    pub fn date(d: &Datetime, f: &str) -> Result<String> {
        match NaiveDate::parse_from_str(d.to_string().as_str(), "%Y-%m-%d") {
            Ok(date) => {
                let formatted = date.format(f).to_string();
                Ok(formatted)
            }
            _ => Ok(d.to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct PageData {
    title: String,
    date: Datetime,
    tags: Vec<String>,
}

#[derive(Template, Deserialize)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub data: PageData,
    pub content: String,
    pub is_xhr: bool,
}

#[derive(Deserialize)]
pub struct ErrorData {
    pub title: String,
}

#[derive(Template, Deserialize)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub data: ErrorData,
    pub content: String,
    pub is_xhr: bool,
}

pub fn unfound_page() -> ErrorTemplate {
    ErrorTemplate {
        data: ErrorData {
            title: String::from("Page not found"),
        },
        content: String::from("Try the [home page](/)"),
        is_xhr: true,
    }
}

impl Default for PageData {
    fn default() -> PageData {
        PageData {
            title: String::new(),
            date: match Datetime::from_str("0000-00-00") {
                Ok(date) => date,
                _ => process::abort(),
            },
            tags: vec![],
        }
    }
}
