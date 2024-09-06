
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{enums::{lang::Lang, route::Route, storage_keys::StorageKey}, services::storage, states::app_state::AppState};

#[function_component(LangComp)]
pub fn lang() -> Html {

    let navigator = use_navigator().unwrap();
    let (state, dispatch) = use_store::<AppState>();
    let state_redi = state.clone();

    let langclick = Callback::from(move |lang: &Lang| {
        // Save the lang in the state
        dispatch.reduce_mut(|state| state.lang = lang.clone());
        // Save the lang in local storage
        storage::save(StorageKey::Lang, lang.to_str());
        // Redirect
        let route = if state_redi.in_game {
            Route::Profil
        } else {
            Route::Login
        };
        navigator.push(&route)
    });
    
    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.lang.to_str());
    
    html! {
        <div class="container">
            <div class="row">
                <h1>{ i18n.t("Language") }</h1>
            </div>
            <div class="row">
                <button onclick={
                    let langclick = langclick.clone();
                    move |_| langclick.emit(&Lang::EN)}>
                    {"English"}    
                </button>
            </div>
            <div class="row">
                <button onclick={
                    let langclick = langclick.clone();
                    move |_| langclick.emit(&Lang::ES)}>
                    {"Español"} 
                </button>
            </div>
            <div class="row">
                <button onclick={
                    let langclick = langclick.clone();
                    move |_| langclick.emit(&Lang::FR)}>
                    {"Français"} 
                </button>
            </div>
            <div class="row">
                <button onclick={
                    let langclick = langclick.clone();
                    move |_| langclick.emit(&Lang::DE)}>
                    {"Deutsch"} 
                </button>
            </div>
        </div>
    }
}