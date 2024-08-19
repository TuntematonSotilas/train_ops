
use yew::prelude::*;
use yewdux::prelude::*;

use crate::states::map_state::MapState;

#[function_component(HudBottComp)]
pub fn hud_bott() -> Html {

    let (state, dispatch) = use_store::<MapState>();

    let buildmode = Callback::from(move |_| {
        dispatch.reduce_mut(|state| state.is_build_mode = !state.is_build_mode);
    });

    html! {
        <div class="hudbott">
            <button onclick={buildmode}>{ "buildmode : " } {state.is_build_mode} </button> 
        </div>
    }
}