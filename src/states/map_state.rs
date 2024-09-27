use yewdux::Store;

#[derive(Default, Copy, Clone, PartialEq, Store)]
pub enum Infra {
    #[default]
    Rail,
    Station,
}

impl Infra {
    pub fn to_tile_class(&self) -> String 
    {
        match &self {
            Infra::Rail => "tile--rail".to_string(),
            Infra::Station => "tile--station".to_string(),
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Store)]
pub struct Tile {
    pub index: usize,
    pub infra: Option<Infra>,
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct MapState {
    pub is_init: bool,
    pub is_menu_build: bool,
    pub is_build_mode: bool,
    pub infra: Infra,
    pub is_drag: bool,
    pub x: i32,
    pub y: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    pub tiles: Vec<Tile>,
}
