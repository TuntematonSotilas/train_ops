
use yew::prelude::*;
use crate::components::{hud_top_comp::HudTopComp, hud_bott_comp::HudBottComp, map_comp::MapComp};

#[function_component(GameComp)]
pub fn game() -> Html {

    html! {
        <div>
            <HudTopComp/>
            <MapComp/> 
            <HudBottComp/>
        </div>
    }
}