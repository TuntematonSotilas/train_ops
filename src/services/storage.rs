use web_sys::Storage;

use crate::enums::storage_keys::StorageKey;

pub fn get(key: StorageKey) -> Option<String> {
    let value = get_storage().get_item(key.to_str()).unwrap();
    value
}

pub fn save(key: StorageKey, value: &str) {
    get_storage().set_item(key.to_str(), value).unwrap();
}

fn get_storage() -> Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}