use base64::decode;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    // log(&encoded_file.into());
    log(&"Greyscale called:".into());

    let base64_to_vector = decode(&encoded_file).unwrap();
    log(&"Image decoded".into());
}
