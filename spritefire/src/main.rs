/*
use spritefire::db::EmojiDatabase;
use std::env;
use std::fs;
*/
use spritefire::build_sprites;

fn main() {
    let test_input = "/Users/joachimpfefferkorn/repos/spritefire/assets/sprites_512";
    _ = build_sprites(&test_input);
    //Ishan's web code
    /*
    let args: Vec<_> = env::args().collect();

    if args[1] == "write" {
        // write db to serialized format
        let emoji_db = EmojiDatabase::from_directory(args[2].clone().into());
        let bytes = emoji_db.to_bytes();
        fs::write(args[3].clone(), bytes).unwrap();
    } else if args[1] == "read" {
        // load db from serialized format
        let bytes = fs::read(args[2].clone()).unwrap();
        let emoji_db = EmojiDatabase::from_bytes(&bytes);
        let img = image::open(args[3].clone()).unwrap();
        let emojis = emoji_db.emojify_image(img, 7);
        for emoji_row in emojis.iter() {
            println!("{}", emoji_row.iter().cloned().collect::<String>());
        }
    }
    */
}
