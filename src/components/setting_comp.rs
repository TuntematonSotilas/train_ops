
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_dispatch;

use crate::{enums::{lang::Lang, route::Route}, states::app_state::AppState};

#[function_component(SettingComp)]
pub fn setting() -> Html {

    let navigator = use_navigator().unwrap();
    let menuclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Menu));

    let dispatch = use_dispatch::<AppState>();

    let langclick = Callback::from(move |lang: &Lang| dispatch.reduce_mut(|state| state.current_lang = lang.clone()));

    html! {
        <div class="container">
            <div class="row">
                <h1>{"Settings"}</h1>
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
                <button onclick={menuclick}>
                    {"Exit"}    
                </button>
            </div>
        </div>
    }
}