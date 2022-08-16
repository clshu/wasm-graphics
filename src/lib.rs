extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn say_hello_from_rust() {
    log("Hello from Rust!");
}

#[wasm_bindgen]
pub struct DougClient {}

#[wasm_bindgen]
impl DougClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("DougClient::new()");
        Self {}
    }

    pub fn update(&self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("DougClient.update()");
        Ok(())
    }

    pub fn render(&self) {
        log("DougClient.render()");
    }
}
