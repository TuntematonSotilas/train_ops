mod app;
mod routes;
pub mod states;
pub mod components;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
