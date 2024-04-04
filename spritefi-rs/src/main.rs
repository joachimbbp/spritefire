use image::GenericImageView;
use image::Rgb;
use spritefi_rs::char_emoji::CharEmoji;
use spritefi_rs::db::EmojiDatabase;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let dir_path = match env::var("DIRECTORY_PATH") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: DIRECTORY_PATH environment variable not set");
            return;
        }
    };
    let emoji_db = EmojiDatabase::from_directory(dir_path.into());
    let symbol = emoji_db.lookup_closest_emoji(Rgb([255, 153, 204]));
    println!("{}", symbol);
}
