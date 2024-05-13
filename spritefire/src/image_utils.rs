use image::{GenericImageView, ImageBuffer, Rgba};
use wgpu::{util::DeviceExt, Queue, Texture};

pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Transform {
    pub scale: f32,
    pub rotation: Coordinates,
    pub translation: Coordinates,
}

#[derive(Copy, Clone, Debug)]
struct Rotation2D {
    //This is probably not the best way to do it
    //I am guessing if you do everything as an array there is a math library that will help you
    tl: f32, //top left
    bl: f32, //bottom left
    tr: f32, //top right
    br: f32, //bottom right
}
impl Rotation2D {
    fn theta(theta: f32) -> Rotation2D {
        Rotation2D {
            tl: theta.cos(),
            bl: theta.sin(),
            tr: -theta.sin(),
            br: theta.cos(),
        }
    }
}

//VERTEX things
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2], // NEW!
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ],
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Corners {
    top_left: (f32, f32),
    bottom_left: (f32, f32),
    bottom_right: (f32, f32),
    top_right: (f32, f32),
}

#[derive(Copy, Clone, Debug)]
struct ImagePlacement {
    corners: Corners,
    vertices: [Vertex; 4],
    indices: [u16; 6],
}

impl ImagePlacement {
    fn new(image_resolution: (u32, u32)) -> ImagePlacement {
        let generated_corners = ImagePlacement::get_corners(image_resolution);
        ImagePlacement {
            corners: generated_corners,
            vertices: ImagePlacement::corners_to_verts(generated_corners),
            indices: [0, 1, 3, 1, 2, 3],
        }
    }
    fn place(
        placement: ImagePlacement,
        scale_factor: f32,
        pos: Coordinates,
        rotation: Rotation2D,
    ) -> ImagePlacement {
        let placement = ImagePlacement::scale(&placement, scale_factor);
        let placement = ImagePlacement::translate(&placement, &pos);
        let placement = ImagePlacement::rotate(&placement, rotation);
        placement
    }
    fn corners_to_verts(corners: Corners) -> [Vertex; 4] {
        [
            Vertex {
                position: [corners.top_left.0, corners.top_left.1, 0.0],
                tex_coords: [0.0, 0.0],
            }, // 0
            Vertex {
                position: [corners.bottom_left.0, corners.bottom_left.1, 0.0],
                tex_coords: [0.0, 1.0],
            }, // 1
            Vertex {
                position: [corners.bottom_right.0, corners.bottom_right.1, 0.0],
                tex_coords: [1.0, 1.],
            }, // 2
            Vertex {
                position: [corners.top_right.0, corners.top_right.1, 0.0],
                tex_coords: [1.0, 0.0],
            }, // 3
        ]
    }
    fn get_corners(image_resolution: (u32, u32)) -> Corners {
        let mut corners = Corners {
            top_left: (0.0, 0.0),
            bottom_left: (0.0, 0.0),
            bottom_right: (0.0, 0.0),
            top_right: (0.0, 0.0),
        };
        corners.top_left = (
            image_resolution.0 as f32 * -0.5,
            image_resolution.1 as f32 * 0.5,
        );
        corners.bottom_left = (
            image_resolution.0 as f32 * -0.5,
            image_resolution.1 as f32 * -0.5,
        );
        corners.bottom_right = (
            image_resolution.0 as f32 * 0.5,
            image_resolution.1 as f32 * -0.5,
        );
        corners.top_right = (
            image_resolution.0 as f32 * 0.5,
            image_resolution.1 as f32 * 0.5,
        );
        corners
    }

    fn scale(input_placement: &ImagePlacement, scale_factor: f32) -> ImagePlacement {
        let mut scaled: ImagePlacement = *input_placement;
        scaled.corners.top_left = (
            scaled.corners.top_left.0 * scale_factor,
            scaled.corners.top_left.1 * scale_factor,
        );
        scaled.corners.bottom_left = (
            scaled.corners.bottom_left.0 * scale_factor,
            scaled.corners.bottom_left.1 * scale_factor,
        );
        scaled.corners.bottom_right = (
            scaled.corners.bottom_right.0 * scale_factor,
            scaled.corners.bottom_right.1 * scale_factor,
        );
        scaled.corners.top_right = (
            scaled.corners.top_right.0 * scale_factor,
            scaled.corners.top_right.1 * scale_factor,
        );
        scaled.vertices = ImagePlacement::corners_to_verts(scaled.corners);
        scaled
    }

    fn translate(current_pos: &ImagePlacement, offset: &Coordinates) -> ImagePlacement {
        //every x value is added to the x offset, and every y to the y
        let mut translated: ImagePlacement = *current_pos;
        translated.corners.top_left = (
            translated.corners.top_left.0 + offset.x,
            translated.corners.top_left.1 + offset.y,
        );
        translated.corners.bottom_left = (
            translated.corners.bottom_left.0 + offset.x,
            translated.corners.bottom_left.1 + offset.y,
        );
        translated.corners.bottom_right = (
            translated.corners.bottom_right.0 + offset.x,
            translated.corners.bottom_right.1 + offset.y,
        );
        translated.corners.top_right = (
            translated.corners.top_right.0 + offset.x,
            translated.corners.top_right.1 + offset.y,
        );
        translated.vertices = ImagePlacement::corners_to_verts(translated.corners);
        translated
    }

    fn rows_x_cols(x: f32, y: f32, matrix: Rotation2D) -> (f32, f32) {
        (matrix.tl * x + matrix.tr * y, matrix.bl * x + matrix.br * y)
    }
    fn rotate(current_pos: &ImagePlacement, rotation: Rotation2D) -> ImagePlacement {
        let mut rotated: ImagePlacement = *current_pos;
        rotated.corners.top_left = ImagePlacement::rows_x_cols(
            rotated.corners.top_left.0,
            rotated.corners.top_left.1,
            rotation,
        );

        rotated.corners.bottom_left = ImagePlacement::rows_x_cols(
            rotated.corners.bottom_left.0,
            rotated.corners.bottom_left.1,
            rotation,
        );
        rotated.corners.top_right = ImagePlacement::rows_x_cols(
            rotated.corners.top_right.0,
            rotated.corners.top_right.1,
            rotation,
        );
        rotated.corners.bottom_right = ImagePlacement::rows_x_cols(
            rotated.corners.bottom_right.0,
            rotated.corners.bottom_right.1,
            rotation,
        );
        rotated.vertices = ImagePlacement::corners_to_verts(rotated.corners);
        rotated
    }
    //THEN YOU CAN HAVE HOOKS INTO TRANSLATION, ROTATION AND SCALE!
    //Way down the line you can do sheer, skew, etc
}

#[derive(Debug)]
pub struct Image {
    pub diffuse_bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Image {
    pub fn load_image(
        path: &str,
        transform: Transform,
        device: &wgpu::Device,
        queue: &Queue,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        diffuse_sampler: &wgpu::Sampler,
    ) -> Image {
        let (diffuse_texture, diffuse_rgba) = ingest_image(path, device);
        //if you included normals etc you'd want a material struct

        let new_image = ImagePlacement::new((diffuse_texture.width(), diffuse_texture.height()));
        let placement = ImagePlacement::place(
            new_image,
            transform.scale,
            transform.translation,
            Rotation2D::theta(transform.rotation.z),
        );

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer2"),
            contents: bytemuck::cast_slice(&placement.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer2"),
            contents: bytemuck::cast_slice(&placement.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = placement.indices.len() as u32; //should always be sixe for a square

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &diffuse_rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * diffuse_texture.width()), //Is calling this function each time computationally expensive?
                rows_per_image: Some(diffuse_texture.height()),
            },
            diffuse_texture.size(),
        );
        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        //return (diffuse_bind_group, vertex_buffer, index_buffer, num_indices);
        Image {
            diffuse_bind_group: diffuse_bind_group,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            num_indices: num_indices,
        }
    }
}

pub fn ingest_image(
    //TODO move to image struct
    image_path: &str,
    device: &wgpu::Device,
) -> (Texture, ImageBuffer<Rgba<u8>, Vec<u8>>) {
    println!("\nImage ingesting...");
    let diffuse_bytes = std::fs::read(image_path).unwrap(); //should be equivelent to include_bytes!()
    let diffuse_image = image::load_from_memory(&diffuse_bytes).unwrap();
    let diffuse_rgba = diffuse_image.to_rgba8();

    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        // All textures are stored as 3D, we represent our 2D texture
        // by setting depth to 1.
        size: wgpu::Extent3d {
            width: diffuse_image.dimensions().0,
            height: diffuse_image.dimensions().1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("diffuse_texture"),
        view_formats: &[],
    });

    (diffuse_texture, diffuse_rgba)
}
