use serde::ser::Error;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Result, Value};

/// Deserializes a JSON string into a vector of a specified type.
///
/// This function takes a JSON string and attempts to deserialize it into a vector
/// of the specified type `T`. It checks if the JSON structure is an array before
/// attempting the deserialization. If the JSON is not an array or if deserialization
/// fails, it returns an error.
///
/// # Type Parameters
/// - `T`: The type of the elements in the resulting vector. Must implement `DeserializeOwned`.
///
/// # Arguments
/// - `json_data`: A string slice containing the JSON data to be deserialized.
///
/// # Returns
/// - `Ok(Vec<T>)`: A vector of elements of type `T` if deserialization is successful.
/// - `Err(Error)`: An error if the JSON is not an array or deserialization fails.
///
/// # Examples
/// ```
/// let json = r#"[{"id": 1, "name": "Item 1"}, {"id": 2, "name": "Item 2"}]"#;
/// let result: Vec<Item> = deserialize_json_array(json).unwrap();
/// ```
pub fn deserialize_json_array<T>(json_data: &str) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let v: Value = serde_json::from_str(json_data)?;
    match v.as_array() {
        Some(array) => serde_json::from_value(Value::Array(array.clone()))
            .map_err(|e| Error::custom(format!("Deserialization error: {}", e))),
        None => Err(Error::custom("JSON data is not an array")),
    }
}

/// Serializes a slice of data into a JSON string.
///
/// This function takes a slice of data that implements `Serialize` and converts it
/// into a JSON string. It is a generic function that works with any data type that
/// implements the `Serialize` trait.
///
/// # Type Parameters
/// - `T`: The type of the elements in the data slice. Must implement `Serialize`.
///
/// # Arguments
/// - `data`: A slice of data to be serialized into JSON.
///
/// # Returns
/// - `Ok(String)`: A `String` containing the JSON representation of the input data.
/// - `Err(Error)`: An error if serialization fails.
///
/// # Examples
/// ```
/// let items = vec!["Item 1", "Item 2", "Item 3"];
/// let json = serialize_json_array(&items).unwrap();
/// ```
pub fn serialize_json_array<T>(data: &[T]) -> Result<String>
where
    T: Serialize,
{
    serde_json::to_string(data)
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_json_array_with_strings() {
        let json = r#"["item1", "item2", "item3"]"#;
        let result: Vec<String> = deserialize_json_array(json).unwrap();
        assert_eq!(result, vec!["item1", "item2", "item3"]);
    }

    #[test]
    fn test_deserialize_json_array_with_numbers() {
        let json = r#"[1, 2, 3]"#;
        let result: Vec<i32> = deserialize_json_array(json).unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_serialize_json_array() {
        let data = vec!["item1", "item2", "item3"];
        let json = serialize_json_array(&data).unwrap();
        assert_eq!(json, json!(data).to_string());
    }

    #[test]
    fn test_deserialize_non_array_json() {
        let json = r#"{"not": "an array"}"#;
        let result: Result<Vec<Value>> = deserialize_json_array(json);
        assert!(result.is_err());
    }
}
