use serde::{Deserialize, Serialize};
use yewdux::Store;
use crate::{enums::{avatar::Avatar, lang::Lang, storage_keys::StorageKey}, services::storage};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,  
    pub user_name: String,
    pub avatar: Avatar,
}

impl User 
{
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).ok().unwrap()
    }

    pub fn from_storage() -> Option<Self> {
        if let Some(json) = storage::get(StorageKey::User) {
            if let Ok(val) = serde_json::from_str::<User>(json.as_str()) {
                return Some(val)
            }
        }
        None
    }
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct AppState {
    pub lang: Lang,
    pub user: Option<User>,
    pub in_game: bool,
}
