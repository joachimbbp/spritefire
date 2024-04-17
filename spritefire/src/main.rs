use spritefire::db::debug_emojis_as_css;
use spritefire::db::EmojiDatabase;
use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args[1] == "write" {
        // write db to serialized format
        let emoji_db = EmojiDatabase::new_from_directory(args[2].clone().into());
        let bytes = emoji_db.to_bytes();
        fs::write(args[3].clone(), bytes).unwrap();
    } else if args[1] == "read" {
        // load db from serialized format
        let bytes = fs::read(args[2].clone()).unwrap();
        let emoji_db = EmojiDatabase::from_bytes(&bytes);
        let img = image::open(args[3].clone()).unwrap();
        let emojis = emoji_db.emojify_image_to_string(img, 4);
        println!("{}", emojis);
    } else if args[1] == "debug" {
        // write average color calculated for emojis to a css file
        // this makes it easy to view in vscode which shows color preview for
        // the hex code
        debug_emojis_as_css(args[2].clone().into())
    }
}
