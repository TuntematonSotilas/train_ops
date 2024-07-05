
use yew::prelude::*;
use yew_router::prelude::*;
use yew_i18n::use_translation;

use crate::enums::route::Route;

#[function_component(MenuComp)]
pub fn menu() -> Html {

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |route: &Route| navigator.push(route));

    let i18n = use_translation();
    
    html! {
        <div class="container">
            <div class="row">
                <h1>{"Menu"}</h1>
            </div>
            <div class="row">
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&Route::Game)}>
                    { i18n.t("New Game") }    
                </button>
            </div>
            <div class="row">
                <button onclick={
                    let onclick = onclick.clone();
                    move |_| onclick.emit(&Route::Setting)}>
                    {"Settings"}
                </button>
            </div>
            
        </div>
    }
}