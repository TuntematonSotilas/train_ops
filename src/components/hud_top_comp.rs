
use yew::prelude::*;
use yew_i18n::use_translation;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::enums::route::Route;
use crate::states::app_state::AppState;

#[function_component(HudTopComp)]
pub fn hud_top() -> Html {

    let navigator = use_navigator().unwrap();
    let exitclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Login));

    let dispatch = use_dispatch::<AppState>();
    let state = dispatch.get();

    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(state.current_lang.to_str());

    html! {
        <div class="hudtop">
            if let Some(user) = &state.user {
                <div>
                    {"User : "}
                    {&user.user_name}
                </div>
            }
            
            <button onclick={exitclick}>{ i18n.t("Exit") }</button> 
        </div>
    }
}