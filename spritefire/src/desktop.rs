use image::DynamicImage;

use crate::db::EmojiDatabase;

pub fn draw(db: EmojiDatabase) {
    println!("database:\n{:#?}", db);
    let img = image::open(
        "/Users/joachimpfefferkorn/repos/spritefire/assets/test_footage/scuba/scuba102.png",
    )
    .unwrap();
    let pool_size = 16;
    println!(
        "Canvas:\n{}",
        EmojiDatabase::emojify_image_to_string(&db, img, pool_size)
    );
}

fn canvas(db: EmojiDatabase, img: DynamicImage) -> String {
    todo!();
    //or maybe just use emojify_image_to_string>
}

//fn draw
//  Takes in image and db
//

// fn emojify_to_canvas
//  basically db::emojify_image_to_string but emojifies to canvas

/*
impl<A: Axis, const K: usize> DistanceMetric<A, K> for SquaredEuclidean {
    #[inline]
    fn dist(a: &[A; K], b: &[A; K]) -> A {
        a.iter()
            .zip(b.iter())
            .map(|(&a_val, &b_val)| {
                let diff: A = a_val.dist(b_val);
                diff * diff
            })
            .fold(A::ZERO, |a, b| a.saturating_add(b))
    }

    #[inline]
    fn dist1(a: A, b: A) -> A {
        let diff: A = a.dist(b);
        diff * diff
    }
}
*/
