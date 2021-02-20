//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_cache::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test() {
    set(String::from("aaaa"), String::from("a"));
    set(String::from("bbbb"), String::from("b"));
    set(String::from("cccc"), String::from("c"));
    assert_eq!(get(String::from("aaaa")), String::from("a"));
    assert_eq!(set(String::from("aaaa"), String::from("b")), String::from("a"));
    assert_eq!(get(String::from("cccc")), String::from("c"));
}
