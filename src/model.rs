use askama::Template;
use serde::Deserialize;

mod filters {
    use askama::Result;
    use pulldown_cmark::{html, Options, Parser};

    pub fn markdown(s: &str) -> Result<String> {
        let mut content = String::new();

        let md_parser = Parser::new_ext(&s, Options::empty());

        html::push_html(&mut content, md_parser);

        Ok(content)
    }
}

#[derive(Deserialize)]
pub struct PageData {
    title: String,
    tags: Vec<String>,
}

#[derive(Template, Deserialize)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub data: PageData,
    pub content: String,
}
