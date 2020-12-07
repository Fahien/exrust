/// @todo 0. Draw 2 windows
/// @todo 1. Make the GUI mode immediate
/// @todo 2. Make window content scrollable
/// @todo 3. Make window resizable
use super::*;

use nalgebra::Matrix4;
use std::ops::Deref;
use std::{convert::From, ops::DerefMut};

struct GuiPipeline {
    program: Program,
    position_loc: i32,
    color_loc: i32,
    uv_loc: i32,
    transform_loc: Option<WebGlUniformLocation>,
    view_loc: Option<WebGlUniformLocation>,
    proj_loc: Option<WebGlUniformLocation>,
    sampler_loc: Option<WebGlUniformLocation>,
}

impl GuiPipeline {
    fn new(gl: &GL) -> Self {
        let vert_src = include_str!("../res/shader/gui.vert.glsl");
        let frag_src = include_str!("../res/shader/gui.frag.glsl");
        let program = Program::new(gl.clone(), vert_src, frag_src);
        program.bind();

        let position_loc = program.get_attrib_loc("in_position");
        let color_loc = program.get_attrib_loc("in_color");
        let uv_loc = program.get_attrib_loc("in_uv");
        let transform_loc = program.get_uniform_loc("transform");
        let view_loc = program.get_uniform_loc("view");
        let proj_loc = program.get_uniform_loc("proj");
        let sampler_loc = program.get_uniform_loc("tex_sampler");

        Self {
            program,
            position_loc,
            color_loc,
            uv_loc,
            transform_loc,
            view_loc,
            proj_loc,
            sampler_loc,
        }
    }

    fn draw(&self, primitive: &Primitive) {
        primitive.bind();
        self.bind_attribs();
        primitive.draw();
    }

    fn draw_char(&self, primitive: &Primitive, c: char) {
        primitive.bind();
        self.bind_char_attribs(c);
        primitive.draw();
    }

    fn bind_attribs(&self) {
        // Position
        // Number of bytes between each vertex element
        let stride = std::mem::size_of::<Vertex>() as i32;
        // Offset of vertex data from the beginning of the buffer
        let offset = 0;

        self.program.gl.vertex_attrib_pointer_with_i32(
            self.position_loc as u32,
            3,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program
            .gl
            .enable_vertex_attrib_array(self.position_loc as u32);

        // Color
        let offset = 3 * std::mem::size_of::<f32>() as i32;
        self.program.gl.vertex_attrib_pointer_with_i32(
            self.color_loc as u32,
            4,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program
            .gl
            .enable_vertex_attrib_array(self.color_loc as u32);

        // Texture coordinates
        let offset = 10 * std::mem::size_of::<f32>() as i32;
        self.program.gl.vertex_attrib_pointer_with_i32(
            self.uv_loc as u32,
            2,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program
            .gl
            .enable_vertex_attrib_array(self.uv_loc as u32)
    }

    fn bind_char_attribs(&self, c: char) {
        let index = (c as u8 + 53 as u8) as i32;

        // Position
        // Number of bytes between each vertex element
        let stride = std::mem::size_of::<FontVertex>() as i32;
        // Offset of vertex data from the beginning of the buffer
        let offset = 0;

        self.program.gl.vertex_attrib_pointer_with_i32(
            self.position_loc as u32,
            3,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program
            .gl
            .enable_vertex_attrib_array(self.position_loc as u32);

        // Color
        let offset = 3 * std::mem::size_of::<f32>() as i32;
        self.program.gl.vertex_attrib_pointer_with_i32(
            self.color_loc as u32,
            4,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program
            .gl
            .enable_vertex_attrib_array(self.color_loc as u32);

        // Texture coordinates
        let stride = std::mem::size_of::<UV>() as i32;
        let offset = 4 * std::mem::size_of::<FontVertex>() as i32 + index * stride * 4;
        self.program.gl.vertex_attrib_pointer_with_i32(
            self.uv_loc as u32,
            2,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program
            .gl
            .enable_vertex_attrib_array(self.uv_loc as u32);
    }

    fn set_transform(&self, transform: &Matrix4<f32>) {
        self.program.gl.uniform_matrix4fv_with_f32_array(
            self.transform_loc.as_ref(),
            false,
            transform.as_slice(),
        )
    }

    fn set_view(&self, view: &Matrix4<f32>) {
        self.program.gl.uniform_matrix4fv_with_f32_array(
            self.view_loc.as_ref(),
            false,
            view.as_slice(),
        )
    }

    fn set_proj(&self, proj: &Matrix4<f32>) {
        self.program.gl.uniform_matrix4fv_with_f32_array(
            self.proj_loc.as_ref(),
            false,
            proj.as_slice(),
        )
    }

    fn set_sampler(&self, texture_unit: i32) {
        self.program
            .gl
            .uniform1i(self.sampler_loc.as_ref(), texture_unit);
    }
}

pub struct Gui {
    width: u32,
    height: u32,

    pipeline: GuiPipeline,

    view: Matrix4<f32>,
    proj: Matrix4<f32>,

    // Generic window background
    quad: Primitive,
    // Generic window title bar
    title_bar: Primitive,
    // Primitive for the shadow
    shadow: Primitive,

    texture: Texture,

    pub windows: Vec<Window>,

    font: Font,

    // Global window title height
    title_height: u32,

    // Index of the window to drag around
    dragging_window: Option<usize>,
}

impl Gui {
    fn create_background(gl: GL) -> Primitive {
        let mut quad = Geometry::<Vertex>::quad();

        quad.vertices[0].uv = [0.0, 1.0 / 4.0];
        quad.vertices[1].uv = [1.0, 1.0 / 4.0];
        quad.vertices[2].uv = [1.0, 2.0 / 4.0];
        quad.vertices[3].uv = [0.0, 2.0 / 4.0];

        Primitive::new(gl, &quad)
    }

    fn create_title_bar(gl: GL) -> Primitive {
        let mut quad = Geometry::<Vertex>::quad();

        quad.vertices[0].uv = [0.0, 0.0 / 4.0];
        quad.vertices[1].uv = [1.0, 0.0 / 4.0];
        quad.vertices[2].uv = [1.0, 1.0 / 4.0];
        quad.vertices[3].uv = [0.0, 1.0 / 4.0];

        Primitive::new(gl, &quad)
    }

    fn create_shadow(gl: GL) -> Primitive {
        let mut quad = Geometry::<Vertex>::quad();

        quad.vertices[0].uv = [0.0, 3.0 / 4.0];
        quad.vertices[1].uv = [1.0, 3.0 / 4.0];
        quad.vertices[2].uv = [1.0, 4.0 / 4.0];
        quad.vertices[3].uv = [0.0, 4.0 / 4.0];

        Primitive::new(gl, &quad)
    }

    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let pipeline = GuiPipeline::new(&gl);

        let view = Isometry3::look_at_rh(
            &Point3::new(0.0, 0.0, 0.5),
            &Point3::origin(),
            &Vector3::new(0.0, 1.0, 0.0),
        )
        .to_homogeneous();

        let proj = nalgebra::Orthographic3::new(0.0, width as f32, height as f32, 0.0, 0.125, 1.0)
            .to_homogeneous();

        let quad = Gui::create_background(gl.clone());
        let title_bar = Gui::create_title_bar(gl.clone());
        let shadow = Gui::create_shadow(gl.clone());

        let pixels = &[
            75, 75, 75, 255, // Title color
            50, 50, 50, 255, // Body color
            255, 0, 0, 255, // Red color
            255, 255, 255, 75, // Shadow color
        ];
        let image = Image::from_raw(pixels, 1, 4);
        let texture = Texture::from_image(gl.clone(), &image);

        let font = Font::new(gl.clone());

        let margin = 3;
        let title_height = font.tile_height + margin * 2;

        Self {
            width,
            height,
            pipeline,
            view,
            proj,
            quad,
            title_bar,
            shadow,
            texture,
            windows: vec![],
            font,
            title_height,
            dragging_window: None,
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    // Returns whether input has been handled or not
    pub fn handle_mouse(&mut self, mouse: &Mouse) -> bool {
        for (i, window) in self.windows.iter_mut().enumerate() {
            let mouse_x = mouse.pos.x;
            let mouse_y = self.height as i32 - mouse.pos.y;

            // Check if we are going to drag a window around
            if mouse.left_click {
                let x_in_title =
                    mouse_x > window.pos.x && mouse_x < window.pos.x + window.width as i32;
                let y_in_title =
                    mouse_y > window.pos.y && mouse_y < window.pos.y + self.title_height as i32;
                // Check whether mouse is inside title bar
                if x_in_title && y_in_title {
                    self.dragging_window = Some(i);
                }
            }

            if !mouse.left_down {
                self.dragging_window = None;
            }

            if let Some(window_index) = self.dragging_window {
                if window_index == i {
                    window.pos.x += mouse.drag.x;
                    window.pos.y -= mouse.drag.y;
                }
            }

            // Check whether mouse is inside the whole window
            let x_in_window =
                mouse_x > window.pos.x && mouse_x < window.pos.x + window.width as i32;
            let y_in_window =
                mouse_y > window.pos.y && mouse_y < window.pos.y + window.height as i32;
            if x_in_window && y_in_window {
                return true;
            }
        }

        false
    }

    pub fn draw(&self) {
        self.pipeline.program.bind();

        self.pipeline.set_view(&self.view);
        self.pipeline.set_proj(&self.proj);

        for window in &self.windows {
            self.draw_window(window);
        }
    }

    fn draw_window(&self, window: &Window) {
        self.pipeline.set_sampler(0);

        self.texture.bind();

        // Title bar
        let transform = Matrix4::identity()
            .append_nonuniform_scaling(&Vector3::new(
                window.width as f32,
                self.title_height as f32,
                0.0,
            ))
            .append_translation(&Vector3::new(window.pos.x as f32, window.pos.y as f32, 0.1));
        self.pipeline.set_transform(&transform);
        self.pipeline.draw(&self.title_bar);

        // Background
        let transform = Matrix4::identity()
            .append_nonuniform_scaling(&Vector3::new(
                window.width as f32,
                window.height as f32,
                0.0,
            ))
            .append_translation(&Vector3::new(window.pos.x as f32, window.pos.y as f32, 0.0));
        self.pipeline.set_transform(&transform);
        self.pipeline.draw(&self.quad);

        // Shadow
        let transform = Matrix4::identity()
            .append_nonuniform_scaling(&Vector3::new(
                window.width as f32 + 2.0,
                window.height as f32 + 2.0,
                0.0,
            ))
            .append_translation(&Vector3::new(
                window.pos.x as f32 - 1.0,
                window.pos.y as f32 - 1.0,
                -0.1,
            ));
        self.pipeline.set_transform(&transform);
        self.pipeline.draw(&self.shadow);

        // Text
        self.font.texture.bind();

        // Draw window title name
        for (i, c) in window.name.chars().enumerate() {
            let transform = Matrix4::identity()
                .append_nonuniform_scaling(&Vector3::new(
                    self.font.tile_width as f32,
                    self.font.tile_height as f32,
                    0.0,
                ))
                .append_translation(&Vector3::new(
                    window.pos.x as f32 + 4.0 + (self.font.tile_width as usize * i) as f32,
                    window.pos.y as f32 + 4.0,
                    0.2,
                ));
            self.pipeline.set_transform(&transform);

            self.pipeline.draw_char(&self.font.primitive, c);
        }

        // Draw window text content
        let mut current_line_x = 0;
        let mut current_line_space_offset = 0;
        let mut offset_y = 0;
        let window_margin = 4;

        for (i, word) in window.text.split(" ").enumerate() {
            let word_len = self.font.tile_width * word.len() as u32;
            current_line_x += word_len;

            let word_end_x = current_line_x + current_line_space_offset + self.font.tile_width;
            let content_size = window.width - window_margin * 2;
            if word_end_x > content_size {
                current_line_x = word_len;
                current_line_space_offset = 0;
                offset_y += self.font.tile_height;
            } else if i > 0 {
                current_line_space_offset += self.font.tile_width;
            }

            for (j, c) in word.chars().enumerate() {
                if c == '\n' {
                    current_line_x = word_len - (1 + j as u32) * self.font.tile_width;
                    current_line_space_offset = 0;
                    offset_y += self.font.tile_height;
                    continue;
                }
                let current_char_x = current_line_space_offset
                    + (current_line_x - word_len)
                    + self.font.tile_width * j as u32;
                let translation_x = window.pos.x + (window_margin + current_char_x) as i32;
                let translation_y =
                    window.pos.y + (window_margin + self.title_height + offset_y) as i32;

                let transform = Matrix4::identity()
                    .append_nonuniform_scaling(&Vector3::new(
                        self.font.tile_width as f32,
                        self.font.tile_height as f32,
                        0.0,
                    ))
                    .append_translation(&Vector3::new(
                        translation_x as f32,
                        translation_y as f32,
                        0.2,
                    ));
                self.pipeline.set_transform(&transform);

                self.pipeline.draw_char(&self.font.primitive, c);
            }
        }
    }
}

pub struct Text {
    pub value: String,
}

impl Text {
    fn new() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self {
            value: String::from(value),
        }
    }
}

impl Deref for Text {
    type Target = String;

    fn deref(&self) -> &String {
        &self.value
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

pub struct Window {
    width: u32,
    height: u32,
    pos: na::Vector2<i32>,
    name: String,

    pub text: Text,
}

impl Window {
    pub const MIN_WIDTH: u32 = 128;
    pub const MIN_HEIGHT: u32 = 128;

    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: std::cmp::max(width, Self::MIN_WIDTH),
            height: std::cmp::max(height, Self::MIN_HEIGHT),
            pos: na::Vector2::new(10, 10),
            name: String::from("Test window"),
            text: Text::from(
                "Content is actually drawn inside the window, even if it is a very long string!",
            ),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = std::cmp::max(Self::MIN_WIDTH, width);
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = std::cmp::max(Self::MIN_HEIGHT, height);
    }
}

#[repr(C)]
struct FontVertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Geometry<FontVertex> {
    fn quad() -> Self {
        let vertices: Vec<FontVertex> = vec![
            // Bottom-left
            FontVertex {
                position: [0.0, 1.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            // Bottom-right
            FontVertex {
                position: [1.0, 1.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            // Top-right
            FontVertex {
                position: [1.0, 0.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            // Top-left
            FontVertex {
                position: [0.0, 0.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        Self { vertices, indices }
    }
}

struct Font {
    texture: Texture,
    tile_width: u32,
    tile_height: u32,
    /// This primitive is using a special vertex buffer with 4 vertices
    /// (pos, color) at the beginning followed by the UVs for all letters.
    primitive: Primitive,
}

impl Font {
    fn create_uvs(
        image_width: u32,
        image_height: u32,
        tile_width: u32,
        tile_height: u32,
    ) -> Vec<UV> {
        let row_count = image_height / tile_height;
        let column_count = image_width / tile_width;

        let expected_column_count = 32;
        assert!(column_count >= expected_column_count);

        let mut uvs: Vec<UV> = vec![];
        uvs.reserve((row_count * expected_column_count * 4) as usize);

        for i in 0..row_count {
            for j in 0..expected_column_count {
                // 4 UVs

                // Bottom-left
                uvs.push([
                    (j * tile_width) as f32 / image_width as f32,
                    (i * tile_height + tile_height) as f32 / image_height as f32,
                ]);
                // Bottom-right
                uvs.push([
                    (j * tile_width + tile_width) as f32 / image_width as f32,
                    (i * tile_height + tile_height) as f32 / image_height as f32,
                ]);
                // Top-right
                uvs.push([
                    (j * tile_width + tile_width) as f32 / image_width as f32,
                    (i * tile_height) as f32 / image_height as f32,
                ]);
                // Top-left
                uvs.push([
                    (j * tile_width) as f32 / image_width as f32,
                    (i * tile_height) as f32 / image_height as f32,
                ]);
            }
        }

        uvs
    }

    pub fn new(gl: GL) -> Self {
        let data = include_bytes!("../res/font/spd.png");
        let image = Image::from_png(data);

        let texture = Texture::from_image(gl.clone(), &image);

        let tile_width = 8;
        let tile_height = 13;
        let uvs = Font::create_uvs(image.width, image.height, tile_width, tile_height);
        let uvs_size = uvs.len() * std::mem::size_of::<UV>();

        // Make a quad with vertices with no UVs
        let quad = Geometry::<FontVertex>::quad();
        let vertices_size = quad.vertices.len() * std::mem::size_of::<FontVertex>();

        // Make a vertex buffer with 4 (position,color) at the beginning, and then all the various UVs
        let mut vertex_buffer = Vec::<u8>::new();
        vertex_buffer.resize(vertices_size + uvs_size, 0);

        // Split it
        let (vb_vertices, vb_uvs) = vertex_buffer.split_at_mut(vertices_size);

        // Copy vertices
        let vertex_buf = unsafe {
            std::slice::from_raw_parts(quad.vertices.as_ptr() as *const u8, vertices_size)
        };
        vb_vertices.copy_from_slice(vertex_buf);

        // Then copy UVs
        let uvs_buf = unsafe { std::slice::from_raw_parts(uvs.as_ptr() as *const u8, uvs_size) };
        vb_uvs.copy_from_slice(uvs_buf);

        let primitive = Primitive::from_raw(gl.clone(), &vertex_buffer, &quad.indices);

        Self {
            texture,
            tile_width,
            tile_height,
            primitive,
        }
    }
}
