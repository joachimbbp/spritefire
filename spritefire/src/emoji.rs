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
        let mut avg_r = 0;
        let mut avg_g = 0;
        let mut avg_b = 0;
        let mut pix_count = 0;

        let (x_dim, y_dim) = image.dimensions();

        for y in 0..y_dim {
            for x in 0..x_dim {
                let pixel = image.get_pixel(x, y).0;

                // Consider pixel if it is not transparent
                if pixel[3] > 0 {
                    pix_count += 1;

                    avg_r += pixel[0] as u64;
                    avg_g += pixel[1] as u64;
                    avg_b += pixel[2] as u64;
                }
            }
        }

        let avg_r = avg_r as f64 / pix_count as f64;
        let avg_g = avg_g as f64 / pix_count as f64;
        let avg_b = avg_b as f64 / pix_count as f64;

        Self {
            symbol,
            color: [avg_r as u8, avg_g as u8, avg_b as u8],
        }
    }
}