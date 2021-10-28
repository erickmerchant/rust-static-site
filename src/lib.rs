// extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate wee_alloc;

mod model;

use crate::model::{ErrorData, ErrorTemplate, PostData, PostTemplate};
use askama::Template;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Result<JsValue, JsValue> {
    // console_error_panic_hook::set_once();

    Ok(JsValue::TRUE)
}

#[wasm_bindgen]
pub fn render(status: u16, text: String) -> Result<JsValue, JsValue> {
    let html = match status {
        200 => {
            let file_parts: Vec<&str> = text.splitn(3, "+++").collect();
            let data: PostData = toml::from_str(match file_parts.get(1) {
                Some(s) => s,
                _ => "",
            })
            .unwrap_or_default();
            let content: String = match file_parts.get(2) {
                Some(s) => s,
                _ => "",
            }
            .to_string();
            let template = PostTemplate {
                data: data,
                content: content,
                is_xhr: true,
            };
            template.render()
        }
        404 => {
            let mut template = ErrorTemplate::error_404();

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

    Ok(match &html {
        Ok(h) => JsValue::from_str(&h.as_str()),
        _ => JsValue::from_str(""),
    })
}
