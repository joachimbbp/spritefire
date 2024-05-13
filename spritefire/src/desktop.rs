use crate::db::EmojiDatabase;
use crate::render::run;
use image::{DynamicImage, GenericImageView, Rgb};
use std::path::PathBuf;
use tokio::runtime::Runtime;
use wgpu::{Device, Queue, SurfaceConfiguration};

pub fn draw_frame(db: EmojiDatabase) {
    let img = image::open(
        "/Users/joachimpfefferkorn/repos/spritefire/assets/test_footage/scuba/scuba102.png",
    )
    .unwrap();
    let sprite_root = "/Users/joachimpfefferkorn/repos/spritefire/assets/sprites_512/";
    let pool_size = 16;
    let _canvas = make_canvas(&db, img, pool_size, &sprite_root);
    render_canvas();
}

fn make_canvas(db: &EmojiDatabase, img: DynamicImage, pool_size: u32, sprite_root: &str) -> String {
    //Very similar to emojify_image_to_string in db.rs
    let (width, height) = img.dimensions();
    let num_squares_x = width / pool_size;
    let num_squares_y = height / pool_size;

    let mut canvas: String = String::new();

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
                canvas.push_str(sprite_root);
                canvas.push_str("blanktile"); //might be an issue here string '' vs string literal ""
                canvas.push_str(",");
            } else {
                let avg_r = (sum_r / pix_count) as u8;
                let avg_g = (sum_g / pix_count) as u8;
                let avg_b = (sum_b / pix_count) as u8;

                let emoji = db.lookup_closest_dense_emoji(Rgb([avg_r, avg_g, avg_b]));
                canvas.push_str(sprite_root);
                canvas.push_str(&emoji);
                canvas.push_str(",");
            }
        }
        canvas.push('\n');
    }
    println!("{:#?}", canvas);
    canvas
}

fn render_canvas() {
    //Need your encoder, textureView, clear color, render pipeline, vector of Image, queue, SurfaceTexture
    let rt = Runtime::new().unwrap();
    let handle = rt.handle();
    handle.block_on(run());
}
