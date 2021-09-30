// extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate wee_alloc;

mod model;

use crate::model::{unfound_page, ErrorData, ErrorTemplate, PageData, PageTemplate};
use askama::Template;
use std::path::Path;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn run() -> Result<JsValue, JsValue> {
    // console_error_panic_hook::set_once();

    Ok(JsValue::TRUE)
}

#[wasm_bindgen]
pub async fn render(pathname: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();

    opts.method("GET");

    let file_stem = Path::new(&pathname).file_stem().unwrap();
    let url = format!("/content/{}.md", file_stem.to_str().unwrap());
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let document = window.document().unwrap();
    let main = document.query_selector("main").unwrap();

    match main {
        Some(element) => {
            if resp.status() == 200 {
                let file_contents = JsFuture::from(resp.text()?).await?.as_string().unwrap();
                let file_parts: Vec<&str> = file_contents.splitn(3, "+++").collect();
                let data: PageData = toml::from_str(file_parts[1]).unwrap();
                let content: String = file_parts[2].to_string();
                let template = PageTemplate {
                    data: data,
                    content: content,
                    is_xhr: true,
                };
                element.set_inner_html(&template.render().unwrap());
            } else if resp.status() == 404 {
                let mut template = unfound_page();

                template.is_xhr = true;

                element.set_inner_html(&template.render().unwrap());
            } else {
                let template = ErrorTemplate {
                    data: ErrorData {
                        title: "Error".to_string(),
                    },
                    content: "Something went wrong.".to_string(),
                    is_xhr: true,
                };
                element.set_inner_html(&template.render().unwrap());
            }
            Ok(JsValue::TRUE)
        }
        None => Ok(JsValue::FALSE),
    }
}
