use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

use crate::{enums::storage_keys::StorageKey, services::storage, states::map_state::Infra};
use crate::states::map_state::{ImagesDatas, Tiles, TILE_SIZE};

const DIRT: &str = "#99863a";
const DARK_DIRT: &str = "#785f28";

pub fn draw_map() {
    
    //log::info!("draw_map");

    let tile_size = TILE_SIZE as i32;
    let rail_size = TILE_SIZE as u32;

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

    let mut cam_x = 0;
    let mut cam_y = 0;
    if let Some(camera_x) = storage::get(StorageKey::CameraX) {
        cam_x = camera_x.parse::<i32>().unwrap();
    }
    if let Some(camera_y) = storage::get(StorageKey::CameraY) {
        cam_y = camera_y.parse::<i32>().unwrap();
    }

    let mut rail_img = None;
    if let Some(sto_img) = ImagesDatas::from_storage() {
        let clamped_buf: Clamped<&[u8]> = Clamped(&sto_img.rail);
        let rail = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, rail_size, rail_size).unwrap();
        rail_img = Some(rail);
    }

    let tiles_sto = Tiles::from_storage();
    if let Some(tiles) = tiles_sto {
        
        let fts = tile_size as f64;

        for (i, row) in tiles.data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let x = i as i32 * tile_size + cam_x;
                let y = j as i32 * tile_size + cam_y;
                
                // Only draw visible tiles 
                if x >= -tile_size && x <= (vw as i32 + tile_size) 
                    && y >= -tile_size && y <= (vh as i32 + tile_size) {
                    
                    let fx = x as f64;
                    let fy = y as f64;
                    
                    ctx.set_fill_style_str(&DIRT);
                    ctx.set_stroke_style_str(&DARK_DIRT);
                    ctx.fill_rect(fx, fy, fts, fts);
                    ctx.stroke_rect(fx, fy, fts, fts);
        
                    if col == &Infra::Rail {
                        
                        if let Some(img) = rail_img.clone() {
                           let res = ctx.put_image_data(&img, fx, fy);
                           if res.is_err() {
                                log::error!("err");
                           }
                        }
                        //ctx.set_fill_style(&BLACK.into());
                        //ctx.fill_rect(x, y + 15., tile_size, 5.);
                    }
                }
                
            }
        }
    }
    
}