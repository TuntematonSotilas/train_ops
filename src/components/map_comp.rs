
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{services::{canvas, canvas_util}, states::{map_state::{MapState, TILE_SIZE}, tile_state::TileState}};

#[function_component(MapComp)]
pub fn map() -> Html {

    let (state, dispatch) = use_store::<MapState>();
    let (tile_state, dispatch_tile) = use_store::<TileState>();

    let tile_state_md = tile_state.clone();
    let tile_state_mv = tile_state.clone();
    let tile_state_tv = tile_state.clone();

    let state_init = state.clone();
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
        if !state_init.is_init  {
            dispatch.reduce_mut(|map| map.is_init = true);
            wasm_bindgen_futures::spawn_local(async move {
                let img_data = canvas_util::fetch_url_binary("/public/img/infra/rail.png".to_string()).await;
                dispatch_tile.reduce_mut(|tile| tile.img_data = img_data);
            
                canvas::draw_map(state_init, tile_state);
            });
        } 
    });

    let mouse_down = Callback::from(move |e: MouseEvent| {
        
        e.prevent_default();

        let state_md = state_md.clone();
        let tile_state_md = tile_state_md.clone();
        let dispatch_md = dispatch_md.clone();

        if state_md.is_build_mode {
            let mut tiles = state_md.tiles;
            let i = ((e.x() - state_md.x) / TILE_SIZE) as usize;
            let j = ((e.y() - state_md.y) / TILE_SIZE) as usize;
            tiles[i][j] = state_md.infra;
            //log::info!("i={0} j={1} infra={2}", i, j, state_md.infra.to_str());
            dispatch_md.reduce_mut(|map| map.tiles = tiles);
            canvas::draw_map(state_md, tile_state_md);
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
    
            let mx = e.movement_x();
            let my = e.movement_y();
            dispatch_mv.reduce_mut(|map| map.x += mx);
            dispatch_mv.reduce_mut(|map| map.y += my);
            let state_draw = state_mv.clone();
            let tile_state_draw = tile_state_mv.clone();

            canvas::draw_map(state_draw, tile_state_draw);
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
            let tile_state_tv = tile_state_tv.clone();
            canvas::draw_map(state_draw, tile_state_tv);

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