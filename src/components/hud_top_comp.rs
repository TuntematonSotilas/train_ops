
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::enums::route::Route;
use crate::states::app_state::AppState;

#[function_component(HudTopComp)]
pub fn hud_top() -> Html {

    let navigator = use_navigator().unwrap();
    let profil_click = Callback::from(move |_| navigator.push(&Route::Profil));

    let dispatch = use_dispatch::<AppState>();
    let state = dispatch.get();

    html! {
        <div class="hudtop">
            <div class="avatar avatar--1" onclick={profil_click}>
                if let Some(user) = &state.user {
                    <div class="avatar__username">{&user.user_name}</div>
                }
            </div>
        </div>
    }
}
