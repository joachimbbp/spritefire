use image::{self, DynamicImage, GenericImageView};

// Define a struct to represent an emoji
#[derive(Debug, Copy, Clone)]
pub struct CharEmoji {
    pub symbol: char,
    pub color: [u8; 3],
}

impl From<(DynamicImage, char)> for CharEmoji {
    fn from(value: (DynamicImage, char)) -> Self {
        let (image, symbol) = value;

        let dimensions = image.dimensions();
        let mut total_r = 0;
        let mut total_g = 0;
        let mut total_b = 0;

        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                let pixel = image.get_pixel(x, y);
                total_r += pixel[0] as u32;
                total_g += pixel[1] as u32;
                total_b += pixel[2] as u32;
            }
        }

        let num_pixels = dimensions.0 * dimensions.1;
        let avg_r = total_r / num_pixels;
        let avg_g = total_g / num_pixels;
        let avg_b = total_b / num_pixels;

        Self {
            symbol,
            color: [avg_r as u8, avg_g as u8, avg_b as u8],
        }
    }
}

impl TryFrom<(DynamicImage, &str)> for CharEmoji {
    type Error = String;

    fn try_from(value: (DynamicImage, &str)) -> Result<Self, Self::Error> {
        let (image, codepoint) = value;
        let mut chars = codepoint.chars();

        if let Some(symbol) = chars.next() {
            if chars.next().is_none() {
                Ok(Self::from((image, symbol)))
            } else {
                Err("String with more than 1 char cannot be parsed into emoji".to_string())
            }
        } else {
            Err("Empty string cannot be parsed into emoji".to_string())
        }
    }
}
