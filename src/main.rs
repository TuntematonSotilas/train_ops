mod app;
mod enums;
pub mod states;
pub mod components;
pub mod services;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    
    console_error_panic_hook::set_once();

    yew::Renderer::<App>::new().render();
}
