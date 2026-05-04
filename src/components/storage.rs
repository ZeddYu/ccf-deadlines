use serde::{Serialize, de::DeserializeOwned};
use web_sys::{console, window};

pub fn get_from_local_storage(key: &str) -> Option<String> {
    window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|local_storage| local_storage.get_item(key).ok().flatten())
}

pub fn get_json_from_local_storage<T: DeserializeOwned>(key: &str) -> Option<T> {
    get_from_local_storage(key).and_then(|data| serde_json::from_str(&data).ok())
}

pub fn set_in_local_storage(key: &str, value: &str) {
    if let Some(local_storage) = window().and_then(|window| window.local_storage().ok().flatten()) {
        if local_storage
            .get_item(key)
            .ok()
            .flatten()
            .is_some_and(|current| current == value)
        {
            return;
        }

        if let Err(error) = local_storage.set_item(key, value) {
            console::warn_1(
                &format!("Failed to write local storage key `{key}`: {error:?}").into(),
            );
        }
    }
}

pub fn set_json_in_local_storage<T: Serialize>(key: &str, value: &T) {
    match serde_json::to_string(value) {
        Ok(serialized) => set_in_local_storage(key, &serialized),
        Err(error) => {
            console::warn_1(
                &format!("Failed to serialize local storage key `{key}`: {error}").into(),
            );
        }
    }
}
