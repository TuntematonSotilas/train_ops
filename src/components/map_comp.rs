
use yew::prelude::*;
use crate::services::canvas;

#[function_component(MapComp)]
pub fn map() -> Html {

    use_effect(|| {
        canvas::draw_map();
    });

    html! {
        <canvas id="map" class="map"></canvas>
    }
}