use crate::db::EmojiDatabase;
use crate::image_utils;
use crate::render::run;
use image::{DynamicImage, GenericImageView, Rgb};
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
pub struct PlacedSprite {
    pub sprite_path: String,
    pub transform: image_utils::Transform,
}
impl PlacedSprite {
    fn build(sprite_path: String, transform: image_utils::Transform) -> PlacedSprite {
        PlacedSprite {
            sprite_path: sprite_path,
            transform: transform,
        }
    }
}

pub fn draw_frame(db: EmojiDatabase) {
    let img = image::open(
        "/Users/joachimpfefferkorn/repos/spritefire/assets/test_footage/scuba/scuba102.png",
    )
    .unwrap();
    let sprite_root = "/Users/joachimpfefferkorn/repos/spritefire/assets/sprites_512/";
    let pool_size = 120;
    let rt = Runtime::new().unwrap();
    let canvas = rt.block_on(make_canvas(&db, img, pool_size, &sprite_root));
    let handle = rt.handle();
    handle.block_on(run(canvas));
}

async fn make_canvas(
    db: &EmojiDatabase,
    img: DynamicImage,
    pool_size: u32,
    sprite_root: &str,
) -> Vec<PlacedSprite> {
    let (width, height) = img.dimensions();
    let num_squares_x = width / pool_size;
    let num_squares_y = height / pool_size;

    //let mut sprite_path: &str;
    let mut canvas: Vec<PlacedSprite> = vec![];

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
                //TODO
                continue;
            } else {
                let avg_r = (sum_r / pix_count) as u8;
                let avg_g = (sum_g / pix_count) as u8;
                let avg_b = (sum_b / pix_count) as u8;
                //TEMP HACK JUST TO GET WGPU ON IT'S FEET
                let emoji = db.lookup_closest_dense_emoji(Rgb([avg_r, avg_g, avg_b]));
                let unicode_emoji = format!("{:x}", emoji.chars().next().unwrap() as u32);
                let sprite_path = format!("{}emoji_u{}.png", sprite_root, unicode_emoji); //terrible parsing gore omg

                let transform = image_utils::Transform {
                    //TEMP GARBO
                    scale: 0.005,
                    rotation: 0.0,
                    translation: [0.0, 0.0, 0.0],
                };
                canvas.push(PlacedSprite::build(sprite_path, transform));
            }
        }
    }
    canvas
}
