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
            for j in 0..x_dim {
                let pixel = image.get_pixel(j, y).0;

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

#[cfg(test)]
mod tests {
    use std::{fmt::format, fs};

    use image::{GenericImageView, Pixel};

    #[test]
    fn thumbnail() {
        let img =
            image::open("/home/twitu/Code/spritefire/assets/sprites_512/emoji_u1f3aa.png").unwrap();
        dbg!(img.dimensions()); // 512, 512
        const DIMS: [u32; 10] = [512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        let mut avg_r = 0;
        let mut avg_g = 0;
        let mut avg_b = 0;
        let mut pix_count = 0;
        for dim in DIMS {
            let thumb = img.thumbnail(dim, dim);
            let mut colors: String = String::new();

            for y in (0..dim) {
                for j in (0..dim) {
                    let pixel = thumb.get_pixel(j, y).0;
                    colors.push_str(
                        &format!(
                            "{:02x}{:02x}{:02x}{:02x} ",
                            pixel[0], pixel[1], pixel[2], pixel[3]
                        )
                        .to_string(),
                    );

                    if dim == 512 {
                        if pixel[3] > 0 {
                            pix_count += 1;

                            avg_r += pixel[0] as u64;
                            avg_g += pixel[1] as u64;
                            avg_b += pixel[2] as u64;
                        }
                    }
                }
                colors.push('\n');
            }

            fs::write(format!("emoji_{}.txt", dim), colors).unwrap();
            thumb.save(format!("emoji_{}.png", dim)).unwrap();
        }
        dbg!(avg_r, avg_g, avg_b);
        let avg_r = (avg_r as f64 / pix_count as f64) as u8;
        let avg_g = (avg_g as f64 / pix_count as f64) as u8;
        let avg_b = (avg_b as f64 / pix_count as f64) as u8;

        fs::write(
            "emoji_avg.txt",
            format!("{:02x}{:02x}{:02x} ", avg_r, avg_g, avg_b),
        )
        .unwrap();
    }
}
