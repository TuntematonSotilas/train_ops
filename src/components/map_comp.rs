
use gloo_timers::callback::Interval;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{enums::storage_keys::StorageKey, services::{canvas, canvas_util, storage}, states::map_state::{ImagesDatas, MapState, Tiles, TILE_SIZE}};

#[function_component(MapComp)]
pub fn map() -> Html {

    let (state, dispatch) = use_store::<MapState>();

    if !state.is_init {
        
        log::info!("init map");
        dispatch.reduce_mut(|map| map.is_init = true);
        
        storage::save(StorageKey::CameraX, "0");
        storage::save(StorageKey::CameraY, "0");
        storage::save(StorageKey::Tiles, Tiles::new().to_json().as_str());

        wasm_bindgen_futures::spawn_local(async move {
            let rail_data = canvas_util::fetch_url_binary("/public/img/infra/rail.png".to_string()).await;
            let images = ImagesDatas {
                rail: rail_data
            };
            storage::save(StorageKey::ImagesDatas, images.to_json().as_str());
        });
    
        let interval = Interval::new(500, move || {
            canvas::draw_map();
        });
        interval.forget(); 
    }

    let state_md = state.clone();
    let state_mv = state.clone();
    let state_te = state.clone();
    let state_ts = state.clone();
    let state_tv = state.clone();

    let dispatch_md = dispatch.clone();
    let dispatch_mu = dispatch.clone();
    let dispatch_te = dispatch.clone();
    let dispatch_ts = dispatch.clone();
    let dispatch_tv = dispatch.clone();

    let mouse_down = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if state_md.is_build_mode {
            let mut x = 0;
            let mut y = 0;
            if let Some(camera_x) = storage::get(StorageKey::CameraX) {
                x = camera_x.parse::<i32>().unwrap();
            }
            if let Some(camera_y) = storage::get(StorageKey::CameraY) {
                y = camera_y.parse::<i32>().unwrap();
            }
            let tiles_sto = Tiles::from_storage();
            if let Some(mut tiles) = tiles_sto {
                let i = ((e.x() - x) / TILE_SIZE) as usize;
                let j = ((e.y() - y) / TILE_SIZE) as usize;
                tiles.data[i][j] = state_md.infra;
                storage::save(StorageKey::Tiles, tiles.to_json().as_str());
            }
           
        } else {
            dispatch_md.reduce_mut(|map| map.is_drag = true);
        }
    });


    let mouse_up = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        dispatch_mu.reduce_mut(|map| map.is_drag = false);
    });


    let mouse_move = Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        if !state_mv.is_build_mode && state_mv.is_drag {
            let mx = e.movement_x();
            let my = e.movement_y();
            if let Some(camera_x) = storage::get(StorageKey::CameraX) {
                let mut x = camera_x.parse::<i32>().unwrap();
                x += mx;
                storage::save(StorageKey::CameraX, x.to_string().as_str());
            }
            if let Some(camera_y) = storage::get(StorageKey::CameraY) {
                let mut y = camera_y.parse::<i32>().unwrap();
                y += my;
                storage::save(StorageKey::CameraY, y.to_string().as_str());
            }
        }
    });

    
    let touch_start = Callback::from(move |_| {
        if !state_ts.is_build_mode {
            dispatch_ts.reduce_mut(|map| map.is_drag = true);
        }
    });

    let touch_end = Callback::from(move |_| {
        dispatch_te.reduce_mut(|map| map.is_drag = false);
        if !state_te.is_build_mode {    
            dispatch_te.reduce_mut(|map| map.prev_x = 0);
            dispatch_te.reduce_mut(|map| map.prev_y = 0);
        }
    });

    let touch_move = Callback::from(move |e: web_sys::TouchEvent| {
        if !state_tv.is_build_mode && state_tv.is_drag {
            let t = e.touches().get(0).unwrap();
            if state_tv.prev_x > 0 {
                let mx = t.client_x() - state_tv.prev_x;
                if let Some(camera_x) = storage::get(StorageKey::CameraX) {
                    let mut x = camera_x.parse::<i32>().unwrap();
                    x += mx;
                    storage::save(StorageKey::CameraX, x.to_string().as_str());
                }
            }
            if state_tv.prev_y > 0 {
                let my = t.client_y() - state_tv.prev_y;
                if let Some(camera_y) = storage::get(StorageKey::CameraY) {
                    let mut y = camera_y.parse::<i32>().unwrap();
                    y += my;
                    storage::save(StorageKey::CameraY, y.to_string().as_str());
                }
            }
            dispatch_tv.reduce_mut(|map| map.prev_x = t.client_x());
            dispatch_tv.reduce_mut(|map| map.prev_y = t.client_y());
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