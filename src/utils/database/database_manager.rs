use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsValue;
use yew_app::services::local_storage::LocalStorageService;

use crate::utils::local_storage_state_manager::json_array_handler::{
    deserialize_json_array, serialize_json_array,
};

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct MeditationData {}

pub struct DatabaseManager {}

impl DatabaseManager {
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

    fn deserialize_and_read(value: &str) -> Vec<MeditationData> {
        deserialize_json_array(value).unwrap_or_else(|_| Vec::new())
    }

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

    pub fn read_data(database_key: &str) -> Vec<MeditationData> {
        match LocalStorageService::read(database_key) {
            Ok(Some(value)) => Self::deserialize_and_read(&value),
            _ => Vec::new(),
        }
    }
}
