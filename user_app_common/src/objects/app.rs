use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize,Deserialize)]
pub struct MainResponse {
    #[wasm_bindgen(skip)]
    pub hello: String,
}

#[wasm_bindgen]
impl MainResponse {
    #[wasm_bindgen(getter)]
    pub fn hello(&self) -> String {
        self.hello.clone()
    }
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct WasmMainResponse {
    #[wasm_bindgen(skip)]
    pub hello_response: String,
}

#[wasm_bindgen]
impl WasmMainResponse {
    #[wasm_bindgen(getter)]
    pub fn hello_response(&self) -> String {
        self.hello_response.clone()
    }
}

#[wasm_bindgen]
pub fn process_main_response(val: JsValue) -> JsValue {
    let response: MainResponse = serde_wasm_bindgen::from_value(val).unwrap();
    let wasm_response = WasmMainResponse {
        hello_response: response.hello + " back from wasm"
    };
    serde_wasm_bindgen::to_value(&wasm_response).unwrap()
}
