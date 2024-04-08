use image::GenericImageView;
use image::Rgb;
use spritefi_rs::char_emoji::CharEmoji;
use spritefi_rs::db::EmojiDatabase;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    // write db to serialized format
    // let emoji_db = EmojiDatabase::from_directory(args[1].clone().into());
    // let bytes = emoji_db.to_bytes();
    // fs::write(args[2].clone(), bytes).unwrap();

    // load db from serialized format
    let bytes = fs::read(args[1].clone()).unwrap();
    let emoji_db = EmojiDatabase::from_bytes(&bytes);
    let img = image::open(args[2].clone()).unwrap();
    let emojis = emoji_db.emojify_image(img, 7);
    for emoji_row in emojis.iter() {
        println!("{}", emoji_row.iter().cloned().collect::<String>());
    }
}
