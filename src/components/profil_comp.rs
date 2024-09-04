
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::{enums::{route::Route, storage_keys::StorageKey}, services::storage, states::app_state::AppState};

#[function_component(ProfilComp)]
pub fn profil() -> Html {

    
    let navigator = use_navigator().unwrap();
    let navigator_lang = navigator.clone();
    let navigator_logout = navigator.clone();

    let (state, dispatch) = use_store::<AppState>();
        
    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());
    
    let btn_click = Callback::from(move |route: &Route| navigator_lang.push(route));
    let btn_back = btn_click.clone();

    let logout_click = Callback::from(move |_| {
        storage::remove(StorageKey::User);
        dispatch.reduce_mut(|state| state.user = None);
        navigator_logout.push(&Route::Login);
    });

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
                <button onclick={logout_click}>
                    { i18n.t("Logout") }
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