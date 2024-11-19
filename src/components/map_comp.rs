
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{services::canvas, states::map_state::{MapState, TILE_SIZE}};

#[function_component(MapComp)]
pub fn map() -> Html {

    let (state, dispatch) = use_store::<MapState>();
    let state_md = state.clone();
    let state_mu = state.clone();
    let state_mv = state.clone();
    let state_te = state.clone();
    let state_ts = state.clone();
    let state_tv = state.clone();
    let dispatch_md = dispatch.clone();
    let dispatch_mu = dispatch.clone();
    let dispatch_mv = dispatch.clone();
    let dispatch_te = dispatch.clone();
    let dispatch_ts = dispatch.clone();
    let dispatch_tv = dispatch.clone();

    use_effect(move || {
        if !state.is_init  {
            dispatch.reduce_mut(|map| map.is_init = true);
            log::info!("use_effect");
            wasm_bindgen_futures::spawn_local(async move {
                canvas::draw_map(state).await;
            });
        } 
    });

    let mouse_down = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if state_md.is_build_mode {
            let mut tiles = state_md.tiles;
            let i = ((e.x() - state_md.x) / TILE_SIZE) as usize;
            let j = ((e.y() - state_md.y) / TILE_SIZE) as usize;
            //log::info!("i={i} j={j}");
            tiles[i][j] = state_md.infra;
            dispatch_md.reduce_mut(|map| map.tiles = tiles);
            let state_draw = state_md.clone();
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("draw");
                canvas::draw_map(state_draw).await;
            });
        } else {
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
    
            //log::info!("{0}", state_mv.last_render);

            if state_mv.last_render > 5 {
                dispatch_mv.reduce_mut(|map| map.last_render = 0);
                let mx = e.movement_x();
                let my = e.movement_y();
                dispatch_mv.reduce_mut(|map| map.x += mx);
                dispatch_mv.reduce_mut(|map| map.y += my);
                let state_draw = state_mv.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    canvas::draw_map(state_draw).await;
                });
            } else {
                dispatch_mv.reduce_mut(|map| map.last_render = state_mv.last_render + 1);
            }

            
        }
    });

    
    let touch_start = Callback::from(move |_| {
        if !state_ts.is_build_mode {
            dispatch_ts.reduce_mut(|map| map.is_drag = true);
        }
    });

    let touch_end = Callback::from(move |_| {
        if !state_te.is_build_mode {
            dispatch_te.reduce_mut(|map| map.is_drag = false);
            dispatch_te.reduce_mut(|map| map.prev_x = 0);
            dispatch_te.reduce_mut(|map| map.prev_y = 0);
        }
    });

    let touch_move = Callback::from(move |e: web_sys::TouchEvent| {
        if !state_tv.is_build_mode && state_tv.is_drag {
            let t = e.touches().get(0).unwrap();
            if state_tv.prev_x > 0 {
                let mx = t.client_x() - state_tv.prev_x;
                dispatch_tv.reduce_mut(|map| map.x += mx);
            }
            if state_tv.prev_y > 0 {
                let my = t.client_y() - state_tv.prev_y;
                dispatch_tv.reduce_mut(|map| map.y += my);
            }
            dispatch_tv.reduce_mut(|map| map.prev_x = t.client_x());
            dispatch_tv.reduce_mut(|map| map.prev_y = t.client_y());

            let state_draw = state_tv.clone();
            wasm_bindgen_futures::spawn_local(async move {
                canvas::draw_map(state_draw).await;
            });
        }
    });
    
    html! {
        <canvas id="map" class="map" 
            onmousedown={mouse_down}
            onmouseup={mouse_up}
            onmousemove={mouse_move}
            ontouchstart={touch_start}
            ontouchend={touch_end}
            ontouchmove={touch_move}>
        </canvas>
    }
}