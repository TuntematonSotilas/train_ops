use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;
use crate::components::{menu_comp::MenuComp, game_comp::GameComp};

#[function_component(App)]
pub fn app() -> Html {

    html! {
        <main class="container">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </main>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Menu => html! { <MenuComp/> },
        Route::Game => html! { <GameComp/> },
    }
}