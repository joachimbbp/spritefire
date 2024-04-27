use std::{
    fs,
    io::Write,
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
    symbols: Vec<Emoji>,
}

impl EmojiDatabase {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        from_bytes(bytes).unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        to_allocvec(&self).unwrap()
    }

    pub fn from_emojis(emojis: Vec<Emoji>) -> Self {
        let mut kdtree = KdTree::new();
        emojis.iter().enumerate().for_each(|(i, emoji)| {
            let color = [
                FixedU8::from_num(emoji.color[0]),
                FixedU8::from_num(emoji.color[1]),
                FixedU8::from_num(emoji.color[2]),
            ];
            kdtree.add(&color, i as u32);
        });

        Self {
            symbols: emojis,
            kdtree,
        }
    }

    pub fn lookup_closest_dense_emoji(&self, rgb: Rgb<u8>) -> &str {
        let point = [
            FixedU8::from_num(rgb[0]),
            FixedU8::from_num(rgb[1]),
            FixedU8::from_num(rgb[2]),
        ];

        let nearest = self.kdtree.nearest_n::<SquaredEuclidean>(&point, 3);
        let (_, symbol) = nearest
            .iter()
            .map(|item| {
                let emoji = &self.symbols[item.item as usize];
                (emoji.density, &emoji.symbol)
            })
            .max()
            .unwrap();
        symbol
    }

    pub fn lookup_closest_emoji(&self, rgb: Rgb<u8>) -> &str {
        let point = [
            FixedU8::from_num(rgb[0]),
            FixedU8::from_num(rgb[1]),
            FixedU8::from_num(rgb[2]),
        ];

        let index: usize = self.kdtree.nearest_one::<SquaredEuclidean>(&point).item as usize;
        &self.symbols[index].symbol
    }

    pub fn new_from_directory(dir_path: PathBuf) -> Self {
        let emojis = read_emojis_from_directory(dir_path);
        EmojiDatabase::from_emojis(emojis)
    }

    pub fn emojify_image_to_string(&self, img: DynamicImage, pool_size: u32) -> String {
        let (width, height) = img.dimensions();
        let num_squares_x = width / pool_size;
        let num_squares_y = height / pool_size;

        let mut emojis: String = String::new();

        for y in 0..=num_squares_y {
            for x in 0..=num_squares_x {
                let mut sum_r = 0;
                let mut sum_g = 0;
                let mut sum_b = 0;
                let mut pix_count = 0;

                for j in 0..pool_size {
                    for i in 0..pool_size {
                        let px = x * pool_size + i;
                        let py = y * pool_size + j;
                        if px < width && py < height {
                            let pixel = img.get_pixel(px, py);
                            if pixel[3] > 0 {
                                sum_r += pixel[0] as u64;
                                sum_g += pixel[1] as u64;
                                sum_b += pixel[2] as u64;
                                pix_count += 1;
                            }
                        }
                    }
                }

                // do no find emoji for transparent parts of image
                if pix_count == 0 {
                    emojis.push(' ')
                } else {
                    let avg_r = (sum_r / pix_count) as u8;
                    let avg_g = (sum_g / pix_count) as u8;
                    let avg_b = (sum_b / pix_count) as u8;

                    let emoji = self.lookup_closest_dense_emoji(Rgb([avg_r, avg_g, avg_b]));
                    emojis.push_str(&emoji);
                }
            }
            emojis.push('\n');
        }

        emojis
    }
}

pub fn read_emojis_from_directory(dir_path: PathBuf) -> Vec<Emoji> {
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
                let code_point = u32::from_str_radix(part, 16).unwrap();
                let code_point = char::from_u32(code_point).unwrap().to_string();
                emoji_unicode_point.push_str(&code_point);
            }

            let img = image::open(&file_path).unwrap();
            emojis.push(Emoji::from((img, emoji_unicode_point)));
        }
    }

    emojis
}

pub fn debug_emojis_as_css(dir_path: PathBuf) {
    let emojis = read_emojis_from_directory(dir_path);

    let mut file = fs::File::create("debug.css").unwrap();
    file.write(b"body {\n").unwrap();

    for emoji in emojis.iter() {
        file.write(format!("    /* {} */\n", emoji.symbol).as_bytes())
            .unwrap();
        file.write(
            format!(
                "    background-color: #{:02x?}{:02x?}{:02x?};\n",
                emoji.color[0], emoji.color[1], emoji.color[2]
            )
            .as_bytes(),
        )
        .unwrap();
    }

    file.write(b"}\n").unwrap();
}
