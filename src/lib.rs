mod utils;

use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn module_memory() -> JsValue {
    vec![wasm_bindgen::module(), wasm_bindgen::memory()].into()
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Counter {
    val: Arc<AtomicU32>,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_raw(&self) -> *const AtomicU32 {
        Arc::into_raw(self.val.clone())
    }

    pub unsafe fn from_raw(val: *const AtomicU32) -> Self {
        Self {
            val: Arc::from_raw(val),
        }
    }

    pub fn incr(&self) -> u32 {
        self.val.fetch_add(1, Ordering::Relaxed)
    }
}
