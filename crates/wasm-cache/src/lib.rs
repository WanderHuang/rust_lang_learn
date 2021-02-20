mod utils;
extern crate lru;
#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use lru::LruCache;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<LruCache<String, String>> = Mutex::new(LruCache::new(10));
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

/// 设置缓存
#[wasm_bindgen]
pub fn set(key: String, json: String) -> String {
    CACHE.lock().unwrap().put(key, json).unwrap_or(String::from(""))
}

/// 读取缓存
#[wasm_bindgen]
pub fn get(key: String) -> String {
    match CACHE.lock().unwrap().get(&key) {
        Some(value) => value.clone(),
        None => "".to_owned()
    }
}
