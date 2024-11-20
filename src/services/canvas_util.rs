use js_sys::Uint8Array;
use wasm_bindgen::{JsCast, Clamped};
use wasm_bindgen_futures::JsFuture;
use web_sys::ImageData;

const DARK_DIRT_RGBA: [u8; 4] = [153, 134, 58, 255];

pub async fn fetch_url_binary(url: String) -> Option<ImageData> {
    let window = web_sys::window().unwrap(); // Browser window
    let promise = JsFuture::from(window.fetch_with_str(&url)); // File fetch promise
    let result = promise.await; // Await fulfillment of fetch
    if let Ok(res) = result {

        let response: web_sys::Response = res.dyn_into().unwrap(); // Type casting
        let image_data = JsFuture::from(response.array_buffer().unwrap()).await; 
        
        if let Ok(bin) = image_data {
            
            let arr = Uint8Array::new(&bin);
            let altbuf = arr.to_vec();
            // Convert the png encoded bytes to an rgba pixel buffer (given the PNG is actually in 8byte RGBA format).
            let image = image::load_from_memory_with_format(&altbuf, image::ImageFormat::Png).unwrap();
            let mut rgba_image = image.to_rgba8();
             // Set background
            for (_, _, pixel) in rgba_image.enumerate_pixels_mut() {
                if pixel[3] == 0 {
                    *pixel = image::Rgba(DARK_DIRT_RGBA);
                }
            }
            let clamped_buf: Clamped<&[u8]> = Clamped(rgba_image.as_raw());
            let image_data = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, image.width(), image.height()).unwrap();
            return Some(image_data);
        }
    }
    None
}
