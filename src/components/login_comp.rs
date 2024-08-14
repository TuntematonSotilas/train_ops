
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::{api::login_api, enums::{route::Route, storage_keys::StorageKey}, services::storage, states::app_state::{AppState, User}};

#[function_component(LoginComp)]
pub fn login() -> Html {

    let (state, dispatch) = use_store::<AppState>();

    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());

    let user_state = use_state(|| String::new());
    let pwd_state = use_state(|| String::new());
    let error_state = use_state(|| String::new());

    let user = user_state.clone();
    let pwd = pwd_state.clone();
    let err = error_state.clone();
    let disp_login = dispatch.clone();
    let disp_logout = dispatch.clone();
    let disp_load = dispatch.clone();
    let i18nc = i18n.clone();

    if let Some(usersto) = User::from_storage() {
        let disp = disp_load.clone();
        // Save the user in the state
        disp.reduce_mut(|state| state.user = Some(usersto));
    }

    let navigator = use_navigator().unwrap();
    let navigator_sett = navigator.clone();

    let sett_click = Callback::from(move |route: &Route| navigator_sett.push(route));

    let login_click = Callback::from(move |_| {
        let user = user_state.clone();
        let pwd = pwd_state.clone();
        let err = error_state.clone();
        let i18n = i18n.clone();
        let navigator = navigator.clone();
        let disp_login = disp_login.clone();
            
        err.set(String::new());
        wasm_bindgen_futures::spawn_local(async move {
            
            let login_res = login_api::login(user.to_string(), pwd.to_string()).await;
            if let Some(user) = login_res {
                // Save the user in local storage
                storage::save(StorageKey::User, user.to_json().as_str());
                // Save the user in the state
                disp_login.reduce_mut(|state| state.user = Some(user));
                // Redirect
                navigator.push(&Route::Game);
            } else {
                let trad = i18n.t("Username or password invalid");
                err.set(trad);
            }
        });
    });

    let logout_click = Callback::from(move |_| {
        storage::remove(StorageKey::User);
        disp_logout.reduce_mut(|state| state.user = None);
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

    html! {
        
        if let Some(user) = &state.user {
            <div class="login__bckg">
                <div class="container">
                    <div class="row">
                        <h1>{"Train Ops"}</h1>
                    </div>
                    <div class="row">
                        { i18nc.t("Welcome") } 
                        {" : "} 
                        {&user.user_name}
                    </div>
                    <div class="row">
                        <button onclick={logout_click}>
                            { i18nc.t("Logout") }
                        </button>
                    </div>
                    <div class="row login__btn--settings">
                        <button onclick={
                            move |_| sett_click.emit(&Route::Setting)}>
                            { i18nc.t("Settings") }
                        </button>
                    </div>
                </div>
            </div>
        } else {
            <div class="login__bckg">
                <div class="container">
                    <div class="row">
                        <h1>{"Train Ops"}</h1>
                    </div>
                    <div class="row">
                        <input type="text" placeholder={{ i18nc.t("Username") }} oninput={user_oninput} />
                    </div>
                    <div class="row">
                        <input type="password" placeholder={{ i18nc.t("Password") }} oninput={pwd_oninput} />
                    </div>
                    <div class="row error">
                        {{ (*err).clone() }}
                    </div>
                    <div class="row">
                        <button onclick={login_click}>
                            { i18nc.t("Login") }
                        </button>
                    </div>
                    <div class="row login__btn--settings">
                        <button onclick={
                            move |_| sett_click.emit(&Route::Setting)}>
                            { i18nc.t("Settings") }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}