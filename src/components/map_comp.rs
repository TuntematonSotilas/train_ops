
use yew::prelude::*;

#[function_component(MapComp)]
pub fn map() -> Html {
    html! {
        <canvas id="map" class="map"></canvas>
    }
}