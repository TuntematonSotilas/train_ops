use wasm_bindgen::prelude::*;

const MAP_SIZE: i32 = 10;
const TILE_SIZE: f64 = 32.;
const DIRT: &str = "#99863a";
const DARK_DIRT: &str = "#785f28";

pub fn draw_map() {

    log::info!("draw_map");

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
    
    let margin = 50.;

    for column in 0..MAP_SIZE {
        for row in 0..MAP_SIZE {
            let x = margin + column as f64 * TILE_SIZE;
            let y = margin + row as f64 * TILE_SIZE;
            ctx.set_fill_style(&DIRT.into());
            ctx.set_stroke_style(&DARK_DIRT.into());
            log::info!("{0} {1}",x, y);
            ctx.fill_rect(x, y, TILE_SIZE, TILE_SIZE);
            ctx.stroke_rect(x, y, TILE_SIZE, TILE_SIZE);
        }
    }
}