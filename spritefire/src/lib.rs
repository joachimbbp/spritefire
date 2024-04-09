use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

//MVP:
//fn simple_resize
//  resizes every image in a folder to proper spritefire resolution

pub fn build_sprites(input_path: &str) -> std::io::Result<()> {
    let folder = Path::new(input_path);
    if folder.is_dir() {
        for entry in fs::read_dir(folder)? {
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
                simple_resize(&image_path);
            }
        }
    }
    Ok(())
}

fn simple_resize(image_path: &PathBuf) {
    print!("{}", image_path.display().to_string())
}

//Advanced:
//fn alpha background
//  detects background and replaces it with an alpha layer
//fn object object detection
//  Crops out specific subjects, creates thumbnails, names after the object in the frame
