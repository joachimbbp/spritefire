use db::EmojiDatabase;
use image::{load_from_memory_with_format, ImageFormat};
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;

pub mod char_emoji;
pub mod db;

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

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, emogasm!");
}

static DB: OnceLock<EmojiDatabase> = OnceLock::new();

#[wasm_bindgen]
pub fn set_db() {
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
    let emojis = get_db().emojify_image(img, pool_size);

    let mut output = String::new();

    emojis.iter().for_each(|line| {
        line.iter().for_each(|symbol| {
            output.push(*symbol);
        });
        output.push('\n');
    });

    output
}