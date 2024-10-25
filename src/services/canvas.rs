use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::states::map_state::MapState;

const MAP_SIZE: i32 = 30;
const TILE_SIZE: f64 = 32.;
const DIRT: &str = "#99863a";
const DARK_DIRT: &str = "#785f28";

pub fn draw_map(state: Rc<MapState>) {
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("map").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let vw = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let vh = window.inner_height().unwrap().as_f64().unwrap() as u32 - 10;
    canvas.set_width(vw);
    canvas.set_height(vh);
    
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    ctx.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

    let map_x = state.x as f64;
    let map_y = state.y as f64;
    
    log::info!("map_x:{0}", map_x);

    for column in 0..MAP_SIZE {
        for row in 0..MAP_SIZE {
            let x = column as f64 * TILE_SIZE + map_x;
            let y = row as f64 * TILE_SIZE + map_y;
            ctx.set_fill_style(&DIRT.into());
            ctx.set_stroke_style(&DARK_DIRT.into());
            //log::info!("{0} {1}",x, y);
            ctx.fill_rect(x, y, TILE_SIZE, TILE_SIZE);
            ctx.stroke_rect(x, y, TILE_SIZE, TILE_SIZE);
        }
    }
}