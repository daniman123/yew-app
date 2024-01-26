use crate::services::local_storage::LocalStorageService;
use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;

/// Represents the state and behavior associated with local storage operations.
///
/// This struct encapsulates the state and methods required to interact with local storage,
/// such as saving and retrieving values associated with specific keys.
#[derive(Clone)]
pub struct LocalStorageState {
    /// The key used for local storage operations.
    pub input_key: String,
    /// The value to be saved or retrieved from local storage.
    pub input_value: String,
    /// The value retrieved from local storage.
    pub stored_value: String,
}

impl LocalStorageState {
    /// Creates a new instance of `LocalStorageState` with default values.
    pub fn new() -> Self {
        Self {
            input_key: String::new(),
            input_value: String::new(),
            stored_value: String::new(),
        }
    }

    /// Updates the `input_key` state based on user input.
    ///
    /// This method is intended to be used as a callback for input events,
    /// updating the state with the value entered by the user.
    ///
    /// # Arguments
    /// * `e` - The `InputEvent` triggered by user interaction.
    pub fn on_key_change(&mut self, e: InputEvent) {
        self.input_key = e
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    }

    /// Updates the `input_value` state based on user input.
    ///
    /// This method is intended to be used as a callback for input events,
    /// updating the state with the value entered by the user.
    ///
    /// # Arguments
    /// * `e` - The `InputEvent` triggered by user interaction.
    pub fn on_value_change(&mut self, e: InputEvent) {
        self.input_value = e
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
    }

    /// Saves the current `input_key` and `input_value` to local storage.
    ///
    /// This method invokes the `LocalStorageService` to save the current state values
    /// to the browser's local storage. If the operation fails, it logs an error to the console.
    pub fn on_save(&self) {
        let key = self.input_key.as_str();
        let value = self.input_value.as_str();
        LocalStorageService::create(key, value).unwrap_or_else(|err| {
            web_sys::console::log_1(&JsValue::from_str(&format!("Error saving: {:?}", err)));
        });
    }

    /// Loads a value from local storage based on the current `input_key`.
    ///
    /// Retrieves the value associated with the current `input_key` from local storage.
    /// If the key is found, it updates the `stored_value`. If the key is not found,
    /// it sets `stored_value` to "Not found". Logs an error to the console if the operation fails.
    pub fn on_load(&mut self) {
        match LocalStorageService::read(&self.input_key) {
            Ok(Some(value)) => self.stored_value = value,
            Ok(None) => self.stored_value = "Not found".to_string(),
            Err(err) => {
                web_sys::console::log_1(&JsValue::from_str(&format!("Error loading: {:?}", err)));
            }
        }
    }
}
