use yewdux::Store;

#[derive(Clone, PartialEq, Default)]
pub enum Infra {
    #[default]
    Rail,
    Station,
}

#[derive(Default, Copy, Clone, PartialEq, Store)]
pub struct Tile {
    pub index: usize,
    pub is_rail: bool,
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct MapState {
    pub is_init: bool,
    pub is_build_mode: bool,
    pub infra: Infra,
    pub is_drag: bool,
    pub x: i32,
    pub y: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    pub tiles: Vec<Tile>,
}
