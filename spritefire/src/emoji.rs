use image::{self, DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

use crate::avg_color_and_density;

/// Emoji symbol with the average color of its picture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emoji {
    /// Unicode symbol for the emoji
    pub symbol: String,
    /// Average color for emoji
    pub color: [u8; 3],
    /// More pixels with color make for a more dense emoji image
    pub density: u64,
}

impl From<(DynamicImage, String)> for Emoji {
    fn from(value: (DynamicImage, String)) -> Self {
        let (img, symbol) = value;

        let [avg_r, avg_g, avg_b, pix_count] = avg_color_and_density(&img);

        Self {
            symbol,
            color: [avg_r as u8, avg_g as u8, avg_b as u8],
            density: pix_count,
        }
    }
}
