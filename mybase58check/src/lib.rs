mod utils;

use wasm_bindgen::prelude::*;

use base58check::FromBase58Check;
use base58check::ToBase58Check;
// use base58check::FromBase58CheckError;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn decode(data: &str) -> Result<Vec<u8>, JsValue> {
    match data.from_base58check() {
        Err(_) => return Err(JsValue::from("invalid")),
        Ok(r) => return Ok(r.1)
    }
}

#[wasm_bindgen]
pub fn encode(data: Vec<u8>) -> String {
    data.to_base58check(0)
}

#[wasm_bindgen]
pub fn valid_base58check(data: &str) -> bool {
    if let Err(_) = data.from_base58check() {
        return false
    }

    true
}
