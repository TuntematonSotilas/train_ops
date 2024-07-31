
use yew::prelude::*;
use yew_router::prelude::*;
use yew_i18n::use_translation;
use yewdux::prelude::*;

use crate::{enums::route::Route, states::app_state::AppState};

#[function_component(MenuComp)]
pub fn menu() -> Html {

    let dispatch = use_dispatch::<AppState>();
    let state = dispatch.get();

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |route: &Route| navigator.push(route));

    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());

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
                    { i18n.t("Settings") }
                </button>
            </div>
        </div>
    }
}