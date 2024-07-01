use yewdux::Store;

#[derive(Default, Clone, PartialEq, Store)]
pub struct AppStore {
    pub is_paused: bool,
}
