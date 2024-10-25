
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{services::canvas, states::map_state::MapState};

#[function_component(MapComp)]
pub fn map() -> Html {

    use_effect(|| {
        canvas::draw_map();
    });

    let (state, dispatch) = use_store::<MapState>();

    let mouse_down = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if !state.is_build_mode {
            //dispatch_md.reduce_mut(|map| map.is_drag = true);
            //canvas::draw_map();
        }
    });
    
    html! {
        <canvas id="map" class="map" 
            onmousedown={mouse_down}>
        </canvas>
    }
}