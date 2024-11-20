use yewdux::Store;

const MAP_SIZE: usize = 30;
pub const TILE_SIZE: i32 = 48;

#[derive(Default, Copy, Clone, PartialEq, Store)]
pub enum Infra {
    #[default]
    None,
    Rail,
    Station,
}

impl Infra 
{
    pub fn to_str(&self) -> &str 
    {
        match &self {
            Infra::Rail => "Rail",
            Infra::Station => "Station",
            _ => "None",
        }
    }
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct MapState {
    pub is_init: bool,
    pub is_menu_build_open: bool,
    pub is_build_mode: bool,
    pub infra: Infra,
    pub is_drag: bool,
    pub x: i32,
    pub y: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    pub tiles: [[Infra; MAP_SIZE]; MAP_SIZE],
}
