// extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate wee_alloc;

mod model;

use crate::model::{unfound_page, ErrorData, ErrorTemplate, PageData, PageTemplate};
use askama::Template;
use std::path::Path;
use std::process;
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

#[inline]
pub fn unwrap_abort<T>(o: Option<T>) -> T {
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}

#[wasm_bindgen]
pub async fn render(pathname: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();

    opts.method("GET");

    let file_stem = unwrap_abort(Path::new(&pathname).file_stem());
    let url = format!("/content/{}.md", unwrap_abort(file_stem.to_str()));
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = unwrap_abort(web_sys::window());
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into()?;
    let document = unwrap_abort(window.document());
    let main = document.query_selector("main")?;

    match main {
        Some(element) => {
            let html = match resp.status() {
                200 => {
                    let file_contents =
                        unwrap_abort(JsFuture::from(resp.text()?).await?.as_string());
                    let file_parts: Vec<&str> = file_contents.splitn(3, "+++").collect();
                    let data: PageData = toml::from_str(match file_parts.get(1) {
                        Some(s) => s,
                        _ => "",
                    })
                    .unwrap_or_default();
                    let content: String = match file_parts.get(2) {
                        Some(s) => s,
                        _ => "",
                    }
                    .to_string();
                    let template = PageTemplate {
                        data: data,
                        content: content,
                        is_xhr: true,
                    };
                    template.render()
                }
                404 => {
                    let mut template = unfound_page();

                    template.is_xhr = true;

                    template.render()
                }
                _ => {
                    let template = ErrorTemplate {
                        data: ErrorData {
                            title: String::from("Error"),
                        },
                        content: String::from("Something went wrong."),
                        is_xhr: true,
                    };
                    template.render()
                }
            };

            match &html {
                Ok(h) => {
                    element.set_inner_html(&h);
                }
                _ => {
                    process::abort();
                }
            }

            Ok(JsValue::TRUE)
        }
        None => Ok(JsValue::FALSE),
    }
}
