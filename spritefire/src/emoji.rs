use image::{self, DynamicImage, GenericImageView};

/// Emoji symbol with the average color of its picture
#[derive(Debug, Clone)]
pub struct Emoji {
    /// Unicode symbol for the emoji
    pub symbol: String,
    /// Average color for emoji
    pub color: [u8; 3],
}

impl From<(DynamicImage, String)> for Emoji {
    fn from(value: (DynamicImage, String)) -> Self {
        let (image, symbol) = value;

        // use thumbnail technique to find average color
        let thumbnail = image.thumbnail_exact(1, 1);
        let pixel = thumbnail.get_pixel(0, 0);

        let avg_r = pixel[0];
        let avg_g = pixel[1];
        let avg_b = pixel[2];

        Self {
            symbol,
            color: [avg_r as u8, avg_g as u8, avg_b as u8],
        }
    }
}
