use web_sys::wasm_bindgen::JsValue;
use yew_app::services::local_storage::LocalStorageService;

use super::{
    json_array_handler::{deserialize_json_array, serialize_json_array},
    meditation_log::meditation_data_builder::MeditationData,
};

/// A struct representing the data related to meditation.
/// This struct can be serialized and deserialized for storage purposes.

/// Manages database operations for `MeditationData`.
pub struct DatabaseManager {}

impl DatabaseManager {
    /// Serializes `MeditationData` and writes it to local storage.
    ///
    /// # Arguments
    ///
    /// * `data_vector` - A slice of `MeditationData`, representing the current state of data.
    /// * `data` - The `MeditationData` instance to be added.
    /// * `database_key` - The key used for storing the data in local storage.
    ///
    /// This function serializes the updated data vector and writes it to the local storage.
    /// Logs an error message to the web console on failure.
    fn serialize_and_write(
        data_vector: &[MeditationData],
        data: MeditationData,
        database_key: &str,
    ) {
        let mut data_vector = data_vector.to_vec();
        data_vector.push(data);

        match serialize_json_array(&data_vector) {
            Ok(res) => LocalStorageService::create(database_key, &res).unwrap_or_else(|_| {
                web_sys::console::log_1(&JsValue::from_str("Failed to write data"));
            }),
            Err(_) => web_sys::console::log_1(&JsValue::from_str("Serialization failed")),
        };
    }

    /// Deserializes a JSON string into a vector of `MeditationData`.
    ///
    /// # Arguments
    ///
    /// * `value` - A JSON string representing serialized `MeditationData`.
    ///
    /// Returns a vector of `MeditationData` or an empty vector on deserialization failure.
    fn deserialize_and_read(value: &str) -> Vec<MeditationData> {
        deserialize_json_array(value).unwrap_or_else(|_| Vec::new())
    }

    /// Writes `MeditationData` to local storage.
    ///
    /// # Arguments
    ///
    /// * `data` - The `MeditationData` to be written.
    /// * `database_key` - The key used for storing the data in local storage.
    ///
    /// Reads the current data from local storage, updates it with the new data,
    /// and then writes it back to the storage.
    /// Logs an error message to the web console on read failure.
    pub fn write_data(data: MeditationData, database_key: &str) {
        match LocalStorageService::read(database_key) {
            Ok(Some(value)) => {
                let data_vector = Self::deserialize_and_read(&value);
                Self::serialize_and_write(&data_vector, data, database_key);
            }
            Ok(None) => {
                Self::serialize_and_write(&[], data, database_key);
            }
            Err(err) => {
                web_sys::console::log_1(&JsValue::from_str(&format!("Error loading: {:?}", err)));
            }
        }
    }

    /// Reads `MeditationData` from local storage.
    ///
    /// # Arguments
    ///
    /// * `database_key` - The key used for accessing the data in local storage.
    ///
    /// Returns a vector of `MeditationData`.
    /// Returns an empty vector if the data is not found or on read failure.
    pub fn read_data(database_key: &str) -> Vec<MeditationData> {
        match LocalStorageService::read(database_key) {
            Ok(Some(value)) => Self::deserialize_and_read(&value),
            _ => Vec::new(),
        }
    }
}
