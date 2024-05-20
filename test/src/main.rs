use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use web_sys::{
    BaseKeyframe, CompositeOperation, Request, RequestInit, RequestMode, RtcPeerConnection,
    RtcRtpTransceiver, RtcRtpTransceiverDirection, RtcRtpTransceiverInit,
};

pub fn main() {
    unimplemented!()
}

#[wasm_bindgen(start)]
pub fn entry() {
    run().unwrap();
}

fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init("resource.png", &opts)?;
    let request_mode = request.mode();

    let val = document.create_element("p")?;
    val.set_inner_html(&format!("request_mode={request_mode:?}"));
    body.append_child(&val)?;

    let request_mode_js_value = JsValue::from(request_mode);
    let val = document.create_element("p")?;
    val.set_inner_html(&format!("request_mode_js_value={request_mode_js_value:?}"));
    body.append_child(&val)?;

    let request_mode_from_js_value = RequestMode::from_js_value(&request_mode_js_value);
    let val = document.create_element("p")?;
    val.set_inner_html(&format!(
        "request_mode_from_js_value={request_mode_from_js_value:?}"
    ));
    body.append_child(&val)?;

    let mut base_key_frame = BaseKeyframe::new();
    base_key_frame.composite(Some(CompositeOperation::Add));

    let val = document.create_element("p")?;
    val.set_inner_html(&format!("base_key_frame={base_key_frame:?}"));
    body.append_child(&val)?;

    let _response_promise = window.fetch_with_request(&request);

    let mut tr_init: RtcRtpTransceiverInit = RtcRtpTransceiverInit::new();
    tr_init.direction(RtcRtpTransceiverDirection::Sendonly);

    let pc1: RtcPeerConnection = RtcPeerConnection::new().unwrap();

    let tr1: RtcRtpTransceiver = pc1.add_transceiver_with_str_and_init("audio", &tr_init);

    // let as_js_val = JsValue::from(tr1);

    // let val = document.create_element("p")?;
    // val.set_inner_html(&format!("as_js_val={:?}", as_js_val));
    // body.append_child(&val)?;

    // let current_direction_reflect = Reflect::get(&as_js_val, &JsValue::from("currentDirection"))?;

    // let val = document.create_element("p")?;
    // val.set_inner_html(&format!(
    //     "current_direction_reflect={:?}",
    //     current_direction_reflect
    // ));
    // body.append_child(&val)?;

    assert_eq!(tr1.direction(), RtcRtpTransceiverDirection::Sendonly);
    assert_eq!(tr1.current_direction(), None);

    let mut yo = web_sys::GpuTextureViewDescriptor::new();
    yo.format(web_sys::GpuTextureFormat::R8unorm);
    // let format =

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
