
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::enums::route::Route;
use crate::states::app_state::AppState;
use crate::components::map_comp::MapComp;

#[function_component(GameComp)]
pub fn game() -> Html {

    let navigator = use_navigator().unwrap();
    let menuclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Menu));

    let (ostate, dispatch) = use_store::<AppState>();
    let pauseclick = dispatch.reduce_mut_callback(|state| state.is_paused = !state.is_paused);

    html! {
        <div>
            <div class="row">
                <h1>{"Game"}</h1>
            </div>
            <div class="row">
                <button onclick={menuclick}>{"Exit"}</button>
            </div>
            <div class="row">
                <button onclick={pauseclick}>{"Pause"}</button>
            </div>
            <div class="row">
                { &ostate.is_paused }
            </div>
            <MapComp/> 
        </div>
    }
}