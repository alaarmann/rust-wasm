#![feature(proc_macro)]

extern crate sha2;
use sha2::{Digest, Sha512};

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub extern "C" fn create_sha512(a: &str) -> String {
    let mut hasher = Sha512::default();

    hasher.input(a.as_bytes());

    let output = hasher.result();

    let mut result = String::new();
    for element in output.iter() {
        result.push_str(&format!("{:02x}", element));
    }
    return result;
}
