use wasm_bindgen::prelude::*;

use web_sys::{Request, RequestInit};

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::NoCors);
    let request = Request::new_with_str_and_init("resource.png", &opts)?;
    let request_mode = request.mode();

    let val = document.create_element("p")?;
    val.set_inner_html(&format!("request_mode={request_mode:?}"));
    body.append_child(&val)?;

    let _response_promise = window.fetch_with_request(&request);

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
