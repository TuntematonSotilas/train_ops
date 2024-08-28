
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::hooks::use_navigator;
use yewdux::use_dispatch;

use crate::{enums::route::Route, states::app_state::AppState};

#[function_component(ProfilComp)]
pub fn profil() -> Html {

    
    let navigator = use_navigator().unwrap();
    let navigator_lang = navigator.clone();
    
    let dispatch = use_dispatch::<AppState>();
    let state = dispatch.get();
        
    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());
    
    let btn_click = Callback::from(move |route: &Route| navigator_lang.push(route));
    let btn_exit = btn_click.clone();
    let btn_back = btn_click.clone();

    html! {
        <div class="container">
            <div class="row">
                if let Some(user) = &state.user {
                   {&user.user_name}
                }
            </div>
            <div class="row">
                <button onclick={ move |_| btn_click.emit(&Route::Lang) }>
                    { i18n.t("Language") }
                </button>
            </div>
            <div class="row">
                <button onclick={ move |_| btn_exit.emit(&Route::Login)}>
                    { i18n.t("Exit") }
                </button> 
            </div>
            <div class="row">
                <button onclick={ move |_| btn_back.emit(&Route::Game)}>
                    {"X"}
                </button> 
            </div>
        </div>

    }
}