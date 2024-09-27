
use yew::prelude::*;
use yewdux::prelude::*;

use crate::states::map_state::{Infra, MapState};

#[function_component(HudBottComp)]
pub fn hud_bott() -> Html {

    let (state, dispatch) = use_store::<MapState>();
    let disp_view = dispatch.clone();
    let disp_menu_build = dispatch.clone();
    let disp_build = dispatch.clone();

    let view = Callback::from(move |_| {
        disp_view.reduce_mut(|state| state.is_build_mode = false);
    });
    let menu_build = Callback::from(move |_| {
        disp_menu_build.reduce_mut(|state| state.is_menu_build = !state.is_menu_build);
    });
    let build = Callback::from(move |infra: &Infra| {
        disp_build.reduce_mut(|state| state.is_build_mode = true);
        disp_build.reduce_mut(|state| state.infra = infra.clone());
    });
    let build_station = build.clone();

    html! {
        <div class="hudbott">
            if state.is_menu_build {
                <div class="hudbott__toprow">
                    <div class="hudbott__btn" onclick={move |_| build.emit(&Infra::Rail)}>
                        <img class="hudbott__icon" src="/public/img/infra/rail.png"/>
                    </div>
                    <div class="hudbott__btn" onclick={move |_| build_station.emit(&Infra::Station)}>
                        <img class="hudbott__icon" src="/public/img/infra/station.png"/>
                    </div>
                </div>
            }
            <div class="hudbott__mainrow">
                <div class="hudbott__btn" onclick={view}>
                    <img class="hudbott__icon" src="/public/img/icons/map.png"/>
                </div>
                <div class="hudbott__btn" onclick={menu_build}>
                    <img class="hudbott__icon" src="/public/img/infra/rail.png"/>
                </div>
            </div>
        </div>
    }
}