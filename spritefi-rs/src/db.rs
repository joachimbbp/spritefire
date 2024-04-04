use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::char_emoji::CharEmoji;
use fixed::{types::extra::U0, FixedU8};
use image::Rgb;
use kiddo::fixed::{distance::SquaredEuclidean, kdtree::KdTree};

type Fxd = FixedU8<U0>;

pub struct EmojiDatabase {
    kdtree: KdTree<Fxd, u32, 3, 32, u32>,
    symbols: Vec<char>,
}

impl EmojiDatabase {
    pub fn from_emojis(emojis: Vec<CharEmoji>) -> Self {
        let (symbols, colors): (Vec<_>, Vec<_>) = emojis
            .into_iter()
            .map(|CharEmoji { symbol, color }| {
                let [r, g, b] = color;
                (
                    symbol,
                    [
                        FixedU8::from_num(r),
                        FixedU8::from_num(g),
                        FixedU8::from_num(b),
                    ],
                )
            })
            .unzip();

        let mut kdtree = KdTree::new();
        colors.into_iter().enumerate().for_each(|(i, color)| {
            kdtree.add(&color, i as u32);
        });

        // dbg!(&symbols);
        println!("{}", kdtree.size());
        Self { symbols, kdtree }
    }

    pub fn lookup_closest_emoji(&self, rgb: Rgb<u8>) -> char {
        let point = [
            FixedU8::from_num(rgb[0]),
            FixedU8::from_num(rgb[1]),
            FixedU8::from_num(rgb[2]),
        ];

        let nearest = self.kdtree.nearest_one::<SquaredEuclidean>(&point);
        let index = nearest.item as usize;
        self.symbols[index]
    }

    pub fn from_directory(dir_path: PathBuf) -> Self {
        let dir_path = Path::new(&dir_path);
        let entries = match fs::read_dir(dir_path) {
            Ok(val) => val,
            Err(err) => {
                panic!("Error: {}", err);
            }
        };

        let mut emojis: Vec<CharEmoji> = Vec::new();
        for entry in entries {
            let entry = match entry {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    continue;
                }
            };

            let file_path = entry.path();
            let file_stem = file_path.file_stem().unwrap();

            if let Some(file_name_str) = file_stem.to_str() {
                let parts: Vec<&str> = file_name_str.split('_').collect();

                if parts.len() == 2 {
                    if let Some(hex_str) = parts[1].strip_prefix("u") {
                        if let Ok(hex_num) = u32::from_str_radix(hex_str, 16) {
                            let img = image::open(&file_path);
                            let symbol = char::from_u32(hex_num);
                            match (img, symbol) {
                                (Ok(img), Some(symbol)) => {
                                    emojis.push(CharEmoji::from((img, symbol)))
                                }
                                _ => {
                                    eprintln!(
                                        "Cannot add emoji from file {} to database",
                                        file_name_str
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        EmojiDatabase::from_emojis(emojis)
    }
}
