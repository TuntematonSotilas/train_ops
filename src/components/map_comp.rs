
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{services::canvas, states::map_state::MapState};

#[function_component(MapComp)]
pub fn map() -> Html {

    let (state, dispatch) = use_store::<MapState>();
    let state_md = state.clone();
    let state_mu = state.clone();
    let state_mv = state.clone();
    let dispatch_md = dispatch.clone();
    let dispatch_mu = dispatch.clone();
    let dispatch_mv = dispatch.clone();
    
    use_effect(|| {
        canvas::draw_map(state);
    });

    let mouse_down = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if !state_md.is_build_mode {
            dispatch_md.reduce_mut(|map| map.is_drag = true);
        }
    });
    
    let mouse_up = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if !state_mu.is_build_mode {
            dispatch_mu.reduce_mut(|map| map.is_drag = false);
        }
    });

    let mouse_move = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if !state_mv.is_build_mode && state_mv.is_drag {
            let mx = e.movement_x();
            let my = e.movement_y();
            dispatch_mv.reduce_mut(|map| map.x += mx);
            dispatch_mv.reduce_mut(|map| map.y += my);
            let state = state_mv.clone();
            canvas::draw_map(state);
        }
    });

    html! {
        <canvas id="map" class="map" 
            onmousedown={mouse_down}
            onmouseup={mouse_up}
            onmousemove={mouse_move}>
        </canvas>
    }
}