use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&encoded_file.into());
}
