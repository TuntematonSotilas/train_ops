use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

use crate::{enums::storage_keys::StorageKey, services::storage, states::map_state::Infra};
use crate::states::map_state::{ImagesDatas, Tiles, TILE_SIZE};

const DIRT: &str = "#99863a";
const DARK_DIRT: &str = "#785f28";

pub fn draw_map() {
    
    //log::info!("draw_map");

    let tile_size = TILE_SIZE as f64;
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

    let mut map_x = 0.;
    let mut map_y = 0.;
    if let Some(camera_x) = storage::get(StorageKey::CameraX) {
        map_x = camera_x.parse::<f64>().unwrap();
    }
    if let Some(camera_y) = storage::get(StorageKey::CameraY) {
        map_y = camera_y.parse::<f64>().unwrap();
    }

    let mut rail_img = None;
    if let Some(sto_img) = ImagesDatas::from_storage() {
        let clamped_buf: Clamped<&[u8]> = Clamped(&sto_img.rail);
        let rail = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, rail_size, rail_size).unwrap();
        rail_img = Some(rail);
    }

    let tiles_sto = Tiles::from_storage();
    if let Some(tiles) = tiles_sto {
        for (i, row) in tiles.data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let x = i as f64 * tile_size + map_x;
                let y = j as f64 * tile_size + map_y;
                
                ctx.set_fill_style(&DIRT.into());
                ctx.set_stroke_style(&DARK_DIRT.into());
                ctx.fill_rect(x, y, tile_size, tile_size);
                ctx.stroke_rect(x, y, tile_size, tile_size);
    
                if col == &Infra::Rail {

                    if let Some(img) = rail_img.clone() {
                       let res = ctx.put_image_data(&img, x, y);
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