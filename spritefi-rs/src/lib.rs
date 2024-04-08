use std::env;

use db::EmojiDatabase;
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

#[wasm_bindgen]
pub fn run() {
    let args = ["abcd", "sdfasdfs"];
    let emoji_db = EmojiDatabase::from_directory(args[0].clone().into());
    let img = image::open(args[1].clone()).unwrap();
    let emojis = emoji_db.emojify_image(img, 7);
    for emoji_row in emojis.iter() {
        println!("{}", emoji_row.iter().cloned().collect::<String>());
    }
}
