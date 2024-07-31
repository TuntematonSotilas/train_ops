
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{enums::route::Route, states::app_state::AppState};

#[function_component(LoginComp)]
pub fn login() -> Html {

    let dispatch = use_dispatch::<AppState>();
    let state = dispatch.get();

    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());
   
    let navigator = use_navigator().unwrap();
    let settclick = Callback::from(move |route: &Route| navigator.push(route));

       
    html! {
        <div class="container">
            <div class="row">
                <h1>{"Login"}</h1>
            </div>
            <div class="row">
                <input type="text" placeholder={{ i18n.t("Username") }}/>
            </div>
            <div class="row">
                <input type="password" placeholder={{ i18n.t("Password") }}/>
            </div>
            <div class="row">
                <button>
                    { i18n.t("Connect") }
                </button>
            </div>
            <div class="row login__btn--settings">
                <button onclick={
                    let onclick = settclick.clone();
                    move |_| onclick.emit(&Route::Setting)}>
                    { i18n.t("Settings") }
                </button>
            </div>

        </div>
    }
}