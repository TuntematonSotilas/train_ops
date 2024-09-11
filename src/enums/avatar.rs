use std::slice::Iter;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Avatar {
    #[default]
    H1,
    F1,
}

impl Avatar {
    pub fn to_str(&self) -> &str 
    {
        match &self {
            Avatar::H1 => "H1",
            Avatar::F1 => "F1",
        }
    }
    pub fn iterator() -> Iter<'static, Avatar> {
        static AVATARS: [Avatar; 2] = [Avatar::H1, Avatar::F1];
        AVATARS.iter()
    }
}