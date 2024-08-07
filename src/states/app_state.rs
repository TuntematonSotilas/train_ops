use serde::Deserialize;
use yewdux::Store;
use crate::enums::lang::Lang;

#[derive(Default, Clone, PartialEq, Deserialize)]
pub struct User {
    pub id: String,  
    pub user_name: String  
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct AppState {
    pub current_lang: Lang,
    pub user: User
}
