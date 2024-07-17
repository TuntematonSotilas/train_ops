use yewdux::Store;
use crate::enums::lang::Lang;

#[derive(Default, Clone, PartialEq, Store)]
pub struct AppState {
    pub is_paused: bool,
    pub current_lang: Lang,
}
