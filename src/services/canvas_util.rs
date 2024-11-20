use gloo_net::http::Request;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

const DARK_DIRT_RGBA: [u8; 4] = [153, 134, 58, 255];

pub async fn fetch_url_binary(url: String) -> Option<ImageData> {
    // Fetch picture
    let result = Request::get(&url)
        .send()
        .await
        .expect("HTTP request failed");
    let bin = result.binary()
        .await
        .expect("deserialization failed");
    // Convert the png encoded bytes to an rgba pixel buffer (given the PNG is actually in 8byte RGBA format).
    let image = image::load_from_memory_with_format(&bin, image::ImageFormat::Png).unwrap();
    let mut rgba_image = image.to_rgba8();
    // Set background
    for (_, _, pixel) in rgba_image.enumerate_pixels_mut() {
        if pixel[3] == 0 {
            *pixel = image::Rgba(DARK_DIRT_RGBA);
        }
    }
    // Re-convert to image
    let clamped_buf: Clamped<&[u8]> = Clamped(rgba_image.as_raw());
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, image.width(), image.height()).unwrap();
    Some(image_data)
}
