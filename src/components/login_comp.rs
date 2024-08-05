
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{api::login_api, enums::route::Route, states::app_state::AppState};

#[function_component(LoginComp)]
pub fn login() -> Html {

    let dispatch = use_dispatch::<AppState>();
    let state = dispatch.get();

    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());
   
    let navigator = use_navigator().unwrap();
    let sett_click = Callback::from(move |route: &Route| navigator.push(route));
    
    let login_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let res = login_api::login("a".to_string(), "b".to_string()).await;
            log::info!("{0}", res);
        });
    });
       
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
                <button onclick={login_click}>
                    { i18n.t("Connect") }
                </button>
            </div>
            <div class="row login__btn--settings">
                <button onclick={
                    move |_| sett_click.emit(&Route::Setting)}>
                    { i18n.t("Settings") }
                </button>
            </div>

        </div>
    }
}