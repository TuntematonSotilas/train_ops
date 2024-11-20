
use js_sys::Uint8Array;
use yewdux::Store;

#[derive(Default, Clone, Store)]
pub struct TileState {
    pub img_data: Uint8Array,
}

impl PartialEq for TileState {
    fn eq(&self, other: &Self) -> bool {
        self.img_data.length() == other.img_data.length()
    }
}