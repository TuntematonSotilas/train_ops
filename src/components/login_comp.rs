
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::{api::login_api, enums::route::Route, states::app_state::AppState};

#[function_component(LoginComp)]
pub fn login() -> Html {

    
    let dispatch = use_dispatch::<AppState>();
    let state = dispatch.get();
        
    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());

    let user_state = use_state(|| String::new());
    let pwd_state = use_state(|| String::new());
    let error_state = use_state(|| String::new());

    let user = user_state.clone();
    let pwd = pwd_state.clone();
    let err = error_state.clone();
    let i18nc = i18n.clone();

    let login_click = Callback::from(move |_| {
        let user = user_state.clone();
        let pwd = pwd_state.clone();
        let err = error_state.clone();
        let i18n = i18n.clone();

        err.set(String::new());
        wasm_bindgen_futures::spawn_local(async move {
            let login_res = login_api::login(user.to_string(), pwd.to_string()).await;
            if let Some(login) = login_res {
                log::info!("{0}", login.id);
            } else {
                let trad = i18n.t("Username or password invalid");
                err.set(trad);
            }
        });
    });


    let user_oninput = Callback::from(move |e: InputEvent| {
        let target: HtmlInputElement = e
            .target()
            .unwrap()
            .dyn_into()
            .unwrap_throw();
        user.set(target.value());
    });

    let pwd_oninput = Callback::from(move |e: InputEvent| {
        let target: HtmlInputElement = e
            .target()
            .unwrap()
            .dyn_into()
            .unwrap_throw();
        pwd.set(target.value());
    });

   
    let navigator = use_navigator().unwrap();
    let sett_click = Callback::from(move |route: &Route| navigator.push(route));
    
    

    html! {
        <div class="container">
            <div class="row">
                <h1>{ i18nc.t("Connect") }</h1>
            </div>
            <div class="row">
                <input type="text" placeholder={{ i18nc.t("Username") }} oninput={user_oninput} />
            </div>
            <div class="row">
                <input type="password" placeholder={{ i18nc.t("Password") }} oninput={pwd_oninput} />
            </div>
            <div class="row login__error">
                {{ (*err).clone() }}
            </div>
            <div class="row">
                <button onclick={login_click}>
                    { i18nc.t("Connect") }
                </button>
            </div>
            <div class="row login__btn--settings">
                <button onclick={
                    move |_| sett_click.emit(&Route::Setting)}>
                    { i18nc.t("Settings") }
                </button>
            </div>

        </div>
    }
}