use askama::Template;
use serde::Deserialize;
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
        let date = NaiveDate::parse_from_str(d.to_string().as_str(), "%Y-%m-%d").unwrap();
        let formatted = date.format(f).to_string();

        Ok(formatted)
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
            title: "Page not found".to_string(),
        },
        content: "Try the [home page](/)".to_string(),
        is_xhr: true,
    }
}
