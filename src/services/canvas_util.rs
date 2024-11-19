use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, JsCast, Clamped};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, ImageData};

pub async fn fetch_url_binary(url: String) -> Result<Uint8Array, JsValue> {
    let window = web_sys::window().unwrap(); // Browser window
    let promise = JsFuture::from(window.fetch_with_str(&url)); // File fetch promise
    let result = promise.await?; // Await fulfillment of fetch
    let response: web_sys::Response = result.dyn_into().unwrap(); // Type casting
    let image_data = JsFuture::from(response.array_buffer()?).await?; // Get text
    Ok(Uint8Array::new(&image_data))
}

pub async fn draw_img(url: String, x: f64, y: f64, ctx: &CanvasRenderingContext2d) {
    let binary = fetch_url_binary(url).await;
    if let Ok(bin) = binary {
        let altbuf = bin.to_vec();
        // Convert the png encoded bytes to an rgba pixel buffer (given the PNG is actually in 8byte RGBA format).
        let image = image::load_from_memory_with_format(&altbuf, image::ImageFormat::Png).unwrap();
        let rgba_image = image.to_rgba8();
        let clamped_buf: Clamped<&[u8]> = Clamped(rgba_image.as_raw());
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, image.width(), image.height()).unwrap();
        _ = ctx.put_image_data(&image_data, x, y);
    }
}