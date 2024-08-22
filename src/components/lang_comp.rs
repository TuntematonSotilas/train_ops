
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{enums::{lang::Lang, route::Route}, states::app_state::AppState};

#[function_component(LangComp)]
pub fn lang() -> Html {

    let navigator = use_navigator().unwrap();
    let (state, dispatch) = use_store::<AppState>();

    let langclick = Callback::from(move |lang: &Lang| {
        dispatch.reduce_mut(|state| state.current_lang = lang.clone());
        navigator.push(&Route::Login)
    });
    
    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());
    
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