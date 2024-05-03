use std::sync::OnceLock;

use image::{load_from_memory_with_format, ImageFormat, RgbaImage};
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

use spritefire::db::EmojiDatabase;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

static DB: OnceLock<EmojiDatabase> = OnceLock::new();

#[wasm_bindgen]
pub fn set_db() {
    set_panic_hook();

    let bytes = include_bytes!("../db.dat");
    let emoji_db = EmojiDatabase::from_bytes(bytes);
    DB.set(emoji_db).unwrap();
}

fn get_db() -> &'static EmojiDatabase {
    DB.get().unwrap()
}

#[wasm_bindgen]
pub fn process_img(buf: &[u8], pool_size: u32) -> String {
    let img = load_from_memory_with_format(buf, ImageFormat::Png).unwrap();
    get_db().emojify_image_to_string(img, pool_size)
}

#[wasm_bindgen]
pub fn process_img_data(img_data: ImageData, pool_size: u32) -> String {
    let img = RgbaImage::from_raw(img_data.width(), img_data.height(), img_data.data().0).unwrap();
    let img = image::DynamicImage::ImageRgba8(img);
    get_db().emojify_image_to_string(img, pool_size)
}
