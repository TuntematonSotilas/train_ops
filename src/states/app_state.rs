use yewdux::Store;

#[derive(Default, Clone, PartialEq, Store)]
pub struct AppState {
    pub is_paused: bool,
}