use serde::{Deserialize, Serialize};
use yewdux::Store;

use crate::{enums::storage_keys::StorageKey, services::storage};

pub const MAP_SIZE: usize = 20;
pub const TILE_SIZE: i32 = 48;


#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagesDatas {
    pub rail: Vec<u8>
}

impl ImagesDatas {
    
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).ok().unwrap()
    }

    pub fn from_storage() -> Option<Self> {
        if let Some(json) = storage::get(StorageKey::ImagesDatas) {
            if let Ok(val) = serde_json::from_str::<ImagesDatas>(json.as_str()) {
                return Some(val)
            }
        }
        None
    }
}


#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tiles {
    pub data: [[Infra; MAP_SIZE]; MAP_SIZE]
}

impl Tiles {
    pub fn new() -> Self {
        Self {
            data: [[Infra::None; MAP_SIZE]; MAP_SIZE]
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).ok().unwrap()
    }

    pub fn from_storage() -> Option<Self> {
        if let Some(json) = storage::get(StorageKey::Tiles) {
            if let Ok(val) = serde_json::from_str::<Tiles>(json.as_str()) {
                return Some(val)
            }
        }
        None
    }
}

#[derive(Default, Copy, Clone, PartialEq, Store, Serialize, Deserialize)]
pub enum Infra {
    #[default]
    None,
    Rail,
    Station,
}

impl Infra 
{
    pub fn to_str(&self) -> &str 
    {
        match &self {
            Infra::Rail => "Rail",
            Infra::Station => "Station",
            _ => "None",
        }
    }
    pub fn to_string(&self) -> String 
    {
        self.to_str().to_string()
    }
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct MapState {
    pub is_init: bool,
    pub is_menu_build_open: bool,
    pub is_build_mode: bool,
    pub infra: Infra,
    pub is_drag: bool,
    pub prev_x: i32,
    pub prev_y: i32,
}
