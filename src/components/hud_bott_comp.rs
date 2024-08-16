
use yew::prelude::*;

#[function_component(HudBottComp)]
pub fn hud_bott() -> Html {

    let buildmode: Callback<MouseEvent> = Callback::from(move |_| {});

    html! {
        <div class="hudbott">
            <button onclick={buildmode}>{ "buildmode" }</button> 
        </div>
    }
}