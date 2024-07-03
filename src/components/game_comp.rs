
use yew::prelude::*;
use yewdux::prelude::*;
use crate::states::app_state::AppState;

#[function_component(GameComp)]
pub fn game() -> Html {

    let (ostate, dispatch) = use_store::<AppState>();
    let onclick = dispatch.reduce_mut_callback(|state| state.is_paused = !state.is_paused);

    html! {
        <div>
            <div class="row">
                <h1>{"Game"}</h1>
            </div>
            <div class="row">
                <button onclick={onclick}>{"Pause"}</button>
            </div>
            <div class="row">
                { &ostate.is_paused }
            </div>
        </div>
    }
}