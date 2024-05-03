use spritefire::db::EmojiDatabase;
use std::{env, fs, path::Path};

fn main() {
    let db_path = env::var("DB_PATH").unwrap_or("db.dat".to_string());

    if !Path::new(&db_path).exists() {
        let assets_path = env::var("ASSETS_PATH").unwrap_or("../../assets/sprites_512".to_string());

        let emoji_db = EmojiDatabase::new_from_directory((&assets_path).into());
        let bytes = emoji_db.to_bytes();
        fs::write(&db_path, bytes).unwrap();
    }
}
