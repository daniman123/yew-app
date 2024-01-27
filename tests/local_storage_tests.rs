use wasm_bindgen_test::*;
use yew_app::services::local_storage::LocalStorageService;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_create_and_read() {
    let key = "test_key";
    let value = "test_value";

    LocalStorageService::create(key, value).expect("Failed to create item");
    let read_value = LocalStorageService::read(key).expect("Failed to read item");

    assert_eq!(read_value, Some(value.to_string()));
}

#[wasm_bindgen_test]
fn test_delete() {
    let key = "test_key";

    LocalStorageService::delete(key).expect("Failed to delete item");
    let read_value = LocalStorageService::read(key).expect("Failed to read item");

    assert_eq!(read_value, None);
}
