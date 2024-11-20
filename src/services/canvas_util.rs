use js_sys::Uint8Array;
use wasm_bindgen::{JsCast, Clamped};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, ImageData};

const DARK_DIRT_RGBA: [u8; 4] = [153, 134, 58, 255];

pub async fn fetch_url_binary(url: String) -> Option<Uint8Array> {
    let window = web_sys::window().unwrap(); // Browser window
    let promise = JsFuture::from(window.fetch_with_str(&url)); // File fetch promise
    let result = promise.await; // Await fulfillment of fetch
    if let Ok(res) = result {
        let response: web_sys::Response = res.dyn_into().unwrap(); // Type casting
        let image_data = JsFuture::from(response.array_buffer().unwrap()).await; 
        if let Ok(img) = image_data {
            let arr = Uint8Array::new(&img);
            return Some(arr);
        }
    }
    None
}

pub fn draw_img(ctx: &CanvasRenderingContext2d, x: f64, y: f64, binary: Uint8Array) {
    log::info!("draw_img : binary={0}", binary.length());

    let altbuf = binary.to_vec();
    // Convert the png encoded bytes to an rgba pixel buffer (given the PNG is actually in 8byte RGBA format).
    let image = image::load_from_memory_with_format(&altbuf, image::ImageFormat::Png).unwrap();
    let mut rgba_image = image.to_rgba8();
    // Set background
    for (_, _, pixel) in rgba_image.enumerate_pixels_mut() {
        //log::info!("{0}", pixel[0]);
        if pixel[3] == 0 {
            *pixel = image::Rgba(DARK_DIRT_RGBA);
        }
    }
            
    let clamped_buf: Clamped<&[u8]> = Clamped(rgba_image.as_raw());
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, image.width(), image.height()).unwrap();
    _ = ctx.put_image_data(&image_data, x, y);
}