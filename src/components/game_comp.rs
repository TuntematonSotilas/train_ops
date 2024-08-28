
use yew::prelude::*;
use yewdux::use_dispatch;
use crate::{components::{hud_bott_comp::HudBottComp, hud_top_comp::HudTopComp, map_comp::MapComp}, states::app_state::AppState};

#[function_component(GameComp)]
pub fn game() -> Html {

    let dispatch = use_dispatch::<AppState>();
    dispatch.reduce_mut(|state| state.in_game = true);

    html! {
        <div>
            <HudTopComp/>
            <MapComp/> 
            <HudBottComp/>
        </div>
    }
}