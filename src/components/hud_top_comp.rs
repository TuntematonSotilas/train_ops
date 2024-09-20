
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
            <div class="avatar" onclick={profil_click}>
                if let Some(user) = &state.user {
                    <img class="avatar__img" src={format!("/public/img/avatars/{0}.png",&user.avatar.to_str())}/>
                    <div class="avatar__username">{&user.username}</div>
                }
            </div>
        </div>
    }
}
