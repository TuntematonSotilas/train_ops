
use yew::prelude::*;
use yewdux::prelude::*;

use crate::states::map_state::MapState;

#[function_component(HudBottComp)]
pub fn hud_bott() -> Html {

    let (state, dispatch) = use_store::<MapState>();

    let buildmode = Callback::from(move |_| {
        dispatch.reduce_mut(|state| state.is_build_mode = !state.is_build_mode);
    });
    
    let icon = if state.is_build_mode { "map" } else { "rail" };
    html! {
        <div class="hudbott">
            <div class="hudbott__btn" onclick={buildmode}>
                <img class="hudbott__icon" src={format!("/public/img/icons/{0}.png", icon)}/>
            </div>
        </div>
    }
}