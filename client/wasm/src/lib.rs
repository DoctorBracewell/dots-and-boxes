use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main_func() -> String {
    return "Hello!".to_string();
}
