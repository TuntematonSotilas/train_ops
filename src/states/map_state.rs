use yewdux::Store;

#[derive(Default, Copy, Clone, PartialEq, Store)]
pub enum Infra {
    #[default]
    Rail,
    Station,
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
}
