// TODO - create localStorage wrapper

use web_sys::{wasm_bindgen::JsValue, window};

pub struct LocalStorageService;

impl LocalStorageService {
    pub fn create(key: &str, value: &str) -> Result<(), JsValue> {
        let window = window().expect("no global `window` exists");
        let local_storage = window.local_storage()?.expect("no local storage");

        local_storage.set_item(key, value)
    }

    pub fn read(key: &str) -> Result<Option<String>, JsValue> {
        let window = window().expect("no global `window` exists");
        let local_storage = window.local_storage()?.expect("no local storage");

        local_storage.get_item(key)
    }

    pub fn update(key: &str, value: &str) -> Result<(), JsValue> {
        Self::create(key, value)
    }

    pub fn delete(key: &str) -> Result<(), JsValue> {
        let window = window().expect("no global `window` exists");
        let local_storage = window.local_storage()?.expect("no local storage");

        local_storage.remove_item(key)
    }
}
