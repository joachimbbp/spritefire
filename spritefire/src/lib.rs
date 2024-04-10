use image::{DynamicImage, GenericImageView, ImageError};
use std::fs::{self};
use std::path::{Path, PathBuf};

//MVP:
//fn simple_resize
//  resizes every image in a folder to proper spritefire resolution

//TODO:
//struct or func for background removal

enum Aspect {
    Horizontal,
    Vertical,
    Square,
}

struct Crop {
    _aspect: Aspect,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    //u32 is totally overkill but GenericImageView uses it so we're using it
}

impl Crop {
    fn crop_image(
        image_path: &PathBuf,
        crop_boundaries: &Self,
    ) -> Result<DynamicImage, ImageError> {
        //TODO this needs its own return type
        let img = image::open(image_path)?;

        //view xy is top left. This adjusts for that:
        let crop_width = (img.width() - crop_boundaries.width) / 2;
        let crop_height = (img.height() - crop_boundaries.height) / 2;

        let subimage = img
            .view(
                crop_width,
                crop_height,
                crop_boundaries.width,
                crop_boundaries.height,
            )
            .to_image();
        Ok(DynamicImage::ImageRgba8(subimage))
    }

    //Preparation Techniques
    fn simple_prep(image_path: &PathBuf) -> Result<Self, ImageError> {
        let img = image::open(image_path)?;
        let (rx, ry) = img.dimensions();
        let aspect = Self::get_aspect(rx, ry)?;
        let ((x, y), short_side) = match aspect {
            Aspect::Horizontal => (Self::get_center(rx, ry), ry),
            Aspect::Vertical => (Self::get_center(ry, rx), rx),
            Aspect::Square => (Self::get_center(rx, ry), ry),
        };
        //width and height will always be the shortest side
        Ok(Self {
            _aspect: aspect,
            x,
            y,
            width: short_side,
            height: short_side,
        })
    }

    //Utils
    fn get_aspect(rx: u32, ry: u32) -> Result<Aspect, ImageError> {
        Ok(if rx > ry {
            Aspect::Horizontal
        } else if rx < ry {
            Aspect::Vertical
        } else {
            Aspect::Square
        })
    }
    fn get_center(long_side: u32, short_side: u32) -> (u32, u32) {
        if long_side < short_side {
            panic!("long side is shorter than short side")
        }
        let x: u32 = long_side / 2;
        let y: u32 = short_side / 2;
        (x, y)
    }
}

pub fn build_sprites(input: &str, save: &str) -> std::io::Result<()> {
    let input_folder = Path::new(input);
    if input_folder.is_dir() {
        for entry in fs::read_dir(input_folder)? {
            print!("resizing {:?}\n", entry);
            let entry = entry?;
            let image_path = entry.path();

            let is_image = match image_path.extension() {
                Some(image_path) => {
                    let ext = image_path.to_string_lossy().to_lowercase();
                    ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "gif" || ext == "bmp"
                }
                None => false,
            };
            if is_image {
                print!("is image\n");
                //TODO: Open image, pass to resize
                let crop_boundaries = Crop::simple_prep(&image_path).unwrap();
                let cropped_image = Crop::crop_image(&image_path, &crop_boundaries).unwrap();
                let _ = cropped_image.save(save);
                let file_name = image_path.file_name().unwrap();
                let mut save_path = PathBuf::from(save);
                save_path.push(file_name);
                let _ = cropped_image.save(save_path);
                //error handle later
            }
        }
    }
    Ok(())
}

//Advanced:
//fn alpha background
//  detects background and replaces it with an alpha layer
//fn object object detection
//  Crops out specific subjects, creates thumbnails, names after the object in the frame
