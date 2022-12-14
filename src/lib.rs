extern crate wasm_bindgen;
mod gl_setup;

use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

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
pub struct DougClient {
    gl: GL,
}

#[wasm_bindgen]
impl DougClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self { gl: gl }
    }

    pub fn update(&self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }
}
