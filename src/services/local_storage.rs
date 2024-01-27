use web_sys::{wasm_bindgen::JsValue, window};

/// A service for interacting with the browser's local storage.
///
/// The `LocalStorageService` provides methods to create, read, update, and delete key-value pairs in the browser's local storage.
/// This struct uses the `web_sys` crate to interface with Web APIs.
pub struct LocalStorageService;

impl LocalStorageService {
    /// Stores a key-value pair in the local storage.
    ///
    /// # Parameters
    /// - `key`: A string slice representing the key.
    /// - `value`: A string slice representing the value to be stored.
    ///
    /// # Returns
    /// A `Result<(), JsValue>`:
    /// - `Ok(())` if the operation is successful.
    /// - `Err(JsValue)` if an error occurs, e.g., if the `window` or `local_storage` is not available.
    ///
    /// # Panics
    /// Panics if the global `window` object does not exist.
    pub fn create(key: &str, value: &str) -> Result<(), JsValue> {
        let window = window().expect("no global `window` exists");
        let local_storage = window.local_storage()?.expect("no local storage");

        local_storage.set_item(key, value)
    }

    /// Retrieves a value from local storage by key.
    ///
    /// # Parameters
    /// - `key`: A string slice representing the key to retrieve.
    ///
    /// # Returns
    /// A `Result<Option<String>, JsValue>`:
    /// - `Ok(Some(String))` if the key exists with its corresponding value.
    /// - `Ok(None)` if the key does not exist.
    /// - `Err(JsValue)` if an error occurs.
    ///
    /// # Panics
    /// Panics if the global `window` object does not exist.
    pub fn read(key: &str) -> Result<Option<String>, JsValue> {
        let window = window().expect("no global `window` exists");
        let local_storage = window.local_storage()?.expect("no local storage");

        local_storage.get_item(key)
    }

    /// Removes a key-value pair from local storage.
    ///
    /// # Parameters
    /// - `key`: A string slice representing the key to be removed.
    ///
    /// # Returns
    /// A `Result<(), JsValue>`:
    /// - `Ok(())` if the operation is successful.
    /// - `Err(JsValue)` if an error occurs.
    ///
    /// # Panics
    /// Panics if the global `window` object does not exist.
    pub fn delete(key: &str) -> Result<(), JsValue> {
        let window = window().expect("no global `window` exists");
        let local_storage = window.local_storage()?.expect("no local storage");

        local_storage.remove_item(key)
    }
}
