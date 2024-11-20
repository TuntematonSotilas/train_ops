
use web_sys::ImageData;
use yewdux::Store;

#[derive(PartialEq, Default, Clone, Store)]
pub struct TileState {
    pub img_data: Option<ImageData>,
}
