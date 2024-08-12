use yewdux::Store;

#[derive(Default, Copy, Clone, PartialEq, Store)]
pub struct Tile {
    pub index: usize,
    pub is_rail: bool,
}


#[derive(Default, Clone, PartialEq, Store)]
pub struct MapState {
    pub tiles: Vec<Tile>,
    pub is_init: bool,
}
