use image::{DynamicImage, GenericImageView};

pub mod db;
pub mod emoji;

pub fn avg_color_and_density(img: &DynamicImage) -> [u64; 4] {
    // use thumbnail technique to find average color
    let mut avg_r = 0;
    let mut avg_g = 0;
    let mut avg_b = 0;
    let mut pix_count = 0;

    let (x_dim, y_dim) = img.dimensions();

    for y in 0..y_dim {
        for x in 0..x_dim {
            let pixel = img.get_pixel(x, y).0;

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

    [avg_r as u64, avg_g as u64, avg_b as u64, pix_count]
}
