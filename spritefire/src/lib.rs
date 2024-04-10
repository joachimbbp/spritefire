use image::{GenericImageView, ImageError};
use std::fs::{self};
use std::path::{Path, PathBuf};

//MVP:
//fn simple_resize
//  resizes every image in a folder to proper spritefire resolution

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
                let _ = simple_resize(&image_path, &save);
            }
        }
    }
    Ok(())
}

fn simple_resize(image_path: &PathBuf, save: &str) -> Result<(), ImageError> {
    print!("simple resize\n");
    let img = image::open(image_path)?;
    let cropped_image = img.view(50, 50, 50, 50).to_image();
    //view(x,y,width,height)
    cropped_image.save(Path::new(save).join(image_path.file_name().unwrap()))?;
    print!("{:?} resized", img);
    Ok(())
}

//Advanced:
//fn alpha background
//  detects background and replaces it with an alpha layer
//fn object object detection
//  Crops out specific subjects, creates thumbnails, names after the object in the frame
