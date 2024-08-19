
use yew::prelude::*;
use yewdux::prelude::*;

use crate::states::map_state::{MapState, Tile};

const BOARD_LEGHT: usize = 20;

#[function_component(MapComp)]
pub fn map() -> Html {

    let (state, dispatch) = use_store::<MapState>();
    
    let state_tile = state.clone();
    let state_ts = state.clone();
    let state_te = state.clone();
    let state_tv = state.clone();
    let state_md = state.clone();
    let state_mu = state.clone();
    let state_mv = state.clone();

    let dispatch_tile = dispatch.clone();
    let dispatch_ts = dispatch.clone();
    let dispatch_te = dispatch.clone();
    let dispatch_tv = dispatch.clone();
    let dispatch_md = dispatch.clone();
    let dispatch_mu = dispatch.clone();
    let dispatch_mv = dispatch.clone();

    if !state.is_init {
        let nb_tiles = BOARD_LEGHT * BOARD_LEGHT;
        let tiles = vec![Tile::default(); nb_tiles];
        let mut ntiles = Vec::<Tile>::new();
        for (i, tile) in &mut tiles.clone().iter_mut().enumerate() {
            tile.index = i;
            ntiles.push(*tile);
        }
        dispatch.reduce_mut(|map| map.tiles = ntiles);
        dispatch.reduce_mut(|map: &mut MapState| map.is_init = true);
    }

    let tile_click = Callback::from(move |index: usize| {
        if state_tile.is_build_mode {
            dispatch_tile.reduce_mut(|map| map.tiles[index].is_rail = true);
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
        }
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
        }
    });

    html! {
        <div class={classes!("map", state.is_build_mode.then_some("map--buildmode"))}>

            <div id="map" 
                class="map__move"
                style={format!("left: {0}px; top: {1}px", state.x, state.y)}
                onmousedown={mouse_down}
                onmouseup={mouse_up}
                onmousemove={mouse_move}
                ontouchstart={touch_start}
                ontouchend={touch_end}
                ontouchmove={touch_move}>

                <div class="map__rotate">
                    { 
                        state.tiles.iter().map(|tile| {
                            html!{
                                <div class={classes!("tile", tile.is_rail.then_some("tile--rail"))}
                                    onclick={
                                        let tile_click = tile_click.clone();
                                        let tile = *tile;
                                        move |_| tile_click.emit(tile.index)}>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </div>
    }
}