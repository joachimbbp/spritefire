use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::emoji::Emoji;
use fixed::{types::extra::U0, FixedU8};
use image::{DynamicImage, GenericImageView, Rgb};
use kiddo::fixed::{distance::SquaredEuclidean, kdtree::KdTree};
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

type Fxd = FixedU8<U0>;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmojiDatabase {
    kdtree: KdTree<Fxd, u32, 3, 32, u32>,
    symbols: Vec<String>,
}

impl EmojiDatabase {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        from_bytes(bytes).unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        to_allocvec(&self).unwrap()
    }

    pub fn from_emojis(emojis: Vec<Emoji>) -> Self {
        let (symbols, colors): (Vec<_>, Vec<_>) = emojis
            .into_iter()
            .map(|Emoji { symbol, color }| {
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

    pub fn lookup_closest_emoji(&self, rgb: Rgb<u8>) -> &str {
        let point = [
            FixedU8::from_num(rgb[0]),
            FixedU8::from_num(rgb[1]),
            FixedU8::from_num(rgb[2]),
        ];

        let nearest = self.kdtree.nearest_one::<SquaredEuclidean>(&point);
        let index = nearest.item as usize;
        &self.symbols[index]
    }

    pub fn from_directory(dir_path: PathBuf) -> Self {
        let dir_path = Path::new(&dir_path);
        let entries = match fs::read_dir(dir_path) {
            Ok(val) => val,
            Err(err) => {
                panic!("Error: {}", err);
            }
        };

        let mut emojis: Vec<Emoji> = Vec::new();
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
                let mut parts = parts.iter();

                let mut emoji_unicode_point = String::new();
                let _ = parts.next().unwrap();
                let first = parts.next().unwrap().strip_prefix("u").unwrap();
                let code_point = u32::from_str_radix(first, 16).unwrap();
                let code_point = char::from_u32(code_point).unwrap().to_string();
                emoji_unicode_point.push_str(&code_point);

                while let Some(part) = parts.next() {
                    let code_point = u32::from_str_radix(first, 16).unwrap();
                    let code_point = char::from_u32(code_point).unwrap().to_string();
                    emoji_unicode_point.push_str(&code_point);
                }

                let img = image::open(&file_path).unwrap();
                emojis.push(Emoji::from((img, emoji_unicode_point)));
            }
        }

        EmojiDatabase::from_emojis(emojis)
    }

    pub fn emojify_image_to_string(&self, img: DynamicImage, pool_size: u32) -> String {
        let (width, height) = img.dimensions();

        let num_squares_x = width / pool_size;
        let num_squares_y = height / pool_size;

        let mut emojis: String = String::new();

        for y in 0..num_squares_y {
            let mut row: Vec<char> = Vec::new();
            for x in 0..num_squares_x {
                let mut sum_r = 0;
                let mut sum_g = 0;
                let mut sum_b = 0;

                for j in 0..pool_size {
                    for i in 0..pool_size {
                        let px = x * pool_size + i;
                        let py = y * pool_size + j;
                        if px < width && py < height {
                            let pixel = img.get_pixel(px, py);
                            sum_r += pixel[0] as u32;
                            sum_g += pixel[1] as u32;
                            sum_b += pixel[2] as u32;
                        }
                    }
                }

                let num_pixels = pool_size * pool_size;
                let avg_r = (sum_r / num_pixels) as u8;
                let avg_g = (sum_g / num_pixels) as u8;
                let avg_b = (sum_b / num_pixels) as u8;

                emojis.push_str(&self.lookup_closest_emoji(Rgb([avg_r, avg_g, avg_b])));
            }
            emojis.push('\n');
        }

        emojis
    }
}
