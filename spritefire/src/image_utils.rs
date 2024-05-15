use image::{GenericImageView, ImageBuffer, Rgba};
use wgpu::{util::DeviceExt, Queue, Texture};

#[derive(Debug, Clone)]
pub struct ImageDimensions {
    pub resolution: (u32, u32),
    pub aspect_ratio: f32,
}
impl ImageDimensions {
    pub fn build(x: u32, y: u32) -> ImageDimensions {
        ImageDimensions {
            resolution: (x, y),
            aspect_ratio: (x as f32 / y as f32),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub scale: f32,
    pub rotation: f32,
    pub translation: [f32; 3],
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
    tex_coords: [f32; 2],
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
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ImagePlacement {
    vertices: [Vertex; 4],
    indices: [u16; 6],
}

impl ImagePlacement {
    fn new(image_resolution: (u32, u32)) -> ImagePlacement {
        ImagePlacement {
            vertices: ImagePlacement::get_vertices(image_resolution),
            indices: [0, 1, 3, 1, 2, 3],
        }
    }
    fn place(
        placement: ImagePlacement,
        scale_factor: f32,
        pos: [f32; 3],
        rotation: f32,
        output_dimensions: ImageDimensions,
    ) -> ImagePlacement {
        let placement = ImagePlacement::scale(&placement, scale_factor);
        let placement = ImagePlacement::translate(&placement, &pos);
        let placement = ImagePlacement::rotate(&placement, rotation);
        let placement = ImagePlacement::normalize_to_aspect(&placement, output_dimensions);
        placement
    }
    fn get_vertices(image_resolution: (u32, u32)) -> [Vertex; 4] {
        let (width, height) = (image_resolution.0 as f32, image_resolution.1 as f32);
        [
            Vertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            }, // top_left
            Vertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
            }, // bottom_left
            Vertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
            }, // bottom_right
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
            }, // top_right
        ]
    }

    fn rows_x_cols(x: f32, y: f32, theta: f32) -> (f32, f32) {
        let rotation = Rotation2D::theta(theta); //TODO name theta what it is (degrees, radians, quaterions, lets find out?)
        (
            rotation.tl * x + rotation.tr * y,
            rotation.bl * x + rotation.br * y,
        )
    }

    fn normalize_to_aspect(
        input_placement: &ImagePlacement,
        output_dimensions: ImageDimensions,
    ) -> ImagePlacement {
        let mut normalized: ImagePlacement = *input_placement;
        normalized.vertices[0] = Vertex {
            position: [
                input_placement.vertices[0].position[0] / output_dimensions.aspect_ratio,
                input_placement.vertices[0].position[1],
                0.0,
            ],
            tex_coords: input_placement.vertices[0].tex_coords,
        };
        normalized.vertices[1] = Vertex {
            position: [
                input_placement.vertices[1].position[0] / output_dimensions.aspect_ratio,
                input_placement.vertices[1].position[1],
                0.0,
            ],
            tex_coords: input_placement.vertices[1].tex_coords,
        };

        normalized.vertices[2] = Vertex {
            position: [
                input_placement.vertices[2].position[0] / output_dimensions.aspect_ratio,
                input_placement.vertices[2].position[1],
                0.0,
            ],
            tex_coords: input_placement.vertices[2].tex_coords,
        };
        normalized.vertices[3] = Vertex {
            position: [
                input_placement.vertices[3].position[0] / output_dimensions.aspect_ratio,
                input_placement.vertices[3].position[1],
                0.0,
            ],
            tex_coords: input_placement.vertices[3].tex_coords,
        };

        normalized
    }
    fn scale(input_placement: &ImagePlacement, scale_factor: f32) -> ImagePlacement {
        let mut scaled: ImagePlacement = *input_placement;
        scaled.vertices[0] = Vertex {
            position: [
                input_placement.vertices[0].position[0] * scale_factor,
                input_placement.vertices[0].position[1] * scale_factor,
                0.0,
            ],
            tex_coords: input_placement.vertices[0].tex_coords,
        };
        scaled.vertices[1] = Vertex {
            position: [
                input_placement.vertices[1].position[0] * scale_factor,
                input_placement.vertices[1].position[1] * scale_factor,
                0.0,
            ],
            tex_coords: input_placement.vertices[1].tex_coords,
        };
        scaled.vertices[2] = Vertex {
            position: [
                input_placement.vertices[2].position[0] * scale_factor,
                input_placement.vertices[2].position[1] * scale_factor,
                0.0,
            ],
            tex_coords: input_placement.vertices[2].tex_coords,
        };
        scaled.vertices[3] = Vertex {
            position: [
                input_placement.vertices[3].position[0] * scale_factor,
                input_placement.vertices[3].position[1] * scale_factor,
                0.0,
            ],
            tex_coords: input_placement.vertices[3].tex_coords,
        };
        scaled
    }
    fn translate(input_placement: &ImagePlacement, offset: &[f32; 3]) -> ImagePlacement {
        //every x value is added to the x offset, and every y to the y
        let mut translated: ImagePlacement = *input_placement;
        translated.vertices[0] = Vertex {
            position: [
                input_placement.vertices[0].position[0] + offset[0],
                input_placement.vertices[0].position[1] + offset[1],
                0.0,
            ],
            tex_coords: input_placement.vertices[0].tex_coords,
        };
        translated.vertices[1] = Vertex {
            position: [
                input_placement.vertices[1].position[0] + offset[0],
                input_placement.vertices[1].position[1] + offset[1],
                0.0,
            ],
            tex_coords: input_placement.vertices[1].tex_coords,
        };
        translated.vertices[2] = Vertex {
            position: [
                input_placement.vertices[2].position[0] + offset[0],
                input_placement.vertices[2].position[1] + offset[1],
                0.0,
            ],
            tex_coords: input_placement.vertices[2].tex_coords,
        };
        translated.vertices[3] = Vertex {
            position: [
                input_placement.vertices[3].position[0] + offset[0],
                input_placement.vertices[3].position[1] + offset[1],
                0.0,
            ],
            tex_coords: input_placement.vertices[3].tex_coords,
        };
        translated
    }

    fn rotate(input_placement: &ImagePlacement, theta: f32) -> ImagePlacement {
        let mut rotated: ImagePlacement = *input_placement;

        let top_left = ImagePlacement::rows_x_cols(
            input_placement.vertices[0].position[0],
            input_placement.vertices[0].position[1],
            theta,
        );
        rotated.vertices[0] = Vertex {
            position: [top_left.0, top_left.1, 0.0],
            tex_coords: input_placement.vertices[0].tex_coords,
        };

        let bottom_left = ImagePlacement::rows_x_cols(
            input_placement.vertices[1].position[0],
            input_placement.vertices[1].position[1],
            theta,
        );
        rotated.vertices[1] = Vertex {
            position: [bottom_left.0, bottom_left.1, 0.0],
            tex_coords: input_placement.vertices[1].tex_coords,
        };

        let bottom_right = ImagePlacement::rows_x_cols(
            input_placement.vertices[2].position[0],
            input_placement.vertices[2].position[1],
            theta,
        );
        rotated.vertices[2] = Vertex {
            position: [bottom_right.0, bottom_right.1, 0.0],
            tex_coords: input_placement.vertices[2].tex_coords,
        };

        let top_right = ImagePlacement::rows_x_cols(
            input_placement.vertices[3].position[0],
            input_placement.vertices[3].position[1],
            theta,
        );
        rotated.vertices[3] = Vertex {
            position: [top_right.0, top_right.1, 0.0],
            tex_coords: input_placement.vertices[3].tex_coords,
        };
        rotated
    }
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
        output_res: ImageDimensions,
    ) -> Image {
        let (diffuse_texture, diffuse_rgba) = ingest_image(path, device);
        //if you included normals etc you'd want a material struct

        let new_image = ImagePlacement::new((diffuse_texture.width(), diffuse_texture.height()));
        let placement = ImagePlacement::place(
            new_image,
            transform.scale,
            transform.translation,
            transform.rotation,
            output_res,
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
    println!("\nIngesting {}", image_path);
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
    println!(
        "diffuse texture Size: {:#?}x{:#?}",
        diffuse_texture.width(),
        diffuse_texture.height()
    );
    (diffuse_texture, diffuse_rgba)
}
