use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::states::map_state::{Infra, MapState, TILE_SIZE};

const DIRT: &str = "#99863a";
const DARK_DIRT: &str = "#785f28";
const BLACK: &str = "#000000";

pub fn draw_map(state: Rc<MapState>) {
    
    let tile_size = TILE_SIZE as f64;

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

    for (i, row) in state.tiles.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let x = i as f64 * tile_size + map_x;
            let y = j as f64 * tile_size + map_y;
            
            ctx.set_fill_style(&DIRT.into());
            ctx.set_stroke_style(&DARK_DIRT.into());
            ctx.fill_rect(x, y, tile_size, tile_size);
            ctx.stroke_rect(x, y, tile_size, tile_size);


            if col == &Infra::Rail {
                //let img = document.get_element_by_id("rail").unwrap();
                //let a = web_sys::HtmlImageElement::new().unwrap();
                //a.set_src("/public/infra/rail.png");
                //a.set_width(32);
                //a.set_height(32);
                //ctx.draw_image_with_html_image_element(&a, x, y);
                ctx.set_fill_style(&BLACK.into());
                ctx.fill_rect(x, y + 15., tile_size, 5.);
            }
        }
    }
}