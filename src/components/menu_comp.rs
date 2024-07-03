
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(MenuComp)]
pub fn menu() -> Html {

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Game));
    
    html! {
        <div>
            <div class="row">
                <h1>{"Menu"}</h1>
            </div>
            <div class="row">
                <button onclick={onclick}>{"New game"}</button>
            </div>
        </div>
    }
}