/// @todo 0. Iterate through words for a proper line breaking
/// @todo 1. Make the GUI mode immediate
/// @todo 2. Drag the title bar to move the window around
/// @todo 3. Gui needs to capture mouse input
/// @todo 4. Make window resizable
/// @todo 5. Make window content scrollable
use super::*;

use nalgebra::Matrix4;
use std::convert::From;
use std::ops::Deref;

struct GuiPipeline {
    program: Program,
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

        let transform_loc = program.get_uniform_loc("transform");
        let view_loc = program.get_uniform_loc("view");
        let proj_loc = program.get_uniform_loc("proj");
        let sampler_loc = program.get_uniform_loc("tex_sampler");

        Self {
            program,
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

    fn bind_attribs(&self) {
        // Position
        let position_loc = self.program.get_attrib_loc("in_position");

        // Number of bytes between each vertex element
        let stride = std::mem::size_of::<Vertex>() as i32;
        // Offset of vertex data from the beginning of the buffer
        let offset = 0;

        self.program.gl.vertex_attrib_pointer_with_i32(
            position_loc as u32,
            3,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program
            .gl
            .enable_vertex_attrib_array(position_loc as u32);

        // Color
        let color_loc = self.program.get_attrib_loc("in_color");

        let offset = 3 * std::mem::size_of::<f32>() as i32;
        self.program.gl.vertex_attrib_pointer_with_i32(
            color_loc as u32,
            4,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program.gl.enable_vertex_attrib_array(color_loc as u32);

        // Texture coordinates
        let offset = 10 * std::mem::size_of::<f32>() as i32;
        let uv_loc = self.program.get_attrib_loc("in_uv");
        self.program.gl.vertex_attrib_pointer_with_i32(
            uv_loc as u32,
            2,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program.gl.enable_vertex_attrib_array(uv_loc as u32)
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

    // Generic window background
    quad: Primitive,
    // Generic window title bar
    title_bar: Primitive,

    texture: Texture,

    pub windows: Vec<Window>,

    font: Font,
}

impl Gui {
    fn create_background(gl: GL) -> Primitive {
        let mut quad = Geometry::quad();

        quad.vertices[0].uv = [0.0, 1.0];
        quad.vertices[1].uv = [1.0, 1.0];
        quad.vertices[2].uv = [1.0, 0.5];
        quad.vertices[3].uv = [0.0, 0.5];

        Primitive::new(gl, &quad)
    }

    fn create_title_bar(gl: GL) -> Primitive {
        let mut quad = Geometry::quad();

        quad.vertices[0].uv = [0.0, 0.5];
        quad.vertices[1].uv = [1.0, 0.5];
        quad.vertices[2].uv = [1.0, 0.0];
        quad.vertices[3].uv = [0.0, 0.0];

        Primitive::new(gl, &quad)
    }

    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let pipeline = GuiPipeline::new(&gl);

        let quad = Gui::create_background(gl.clone());
        let title_bar = Gui::create_title_bar(gl.clone());

        let pixels = &[
            75, 75, 75, 255, // Title color
            50, 50, 50, 255, // Body color
        ];
        let image = Image::from_raw(pixels, 1, 2);
        let texture = Texture::from_image(gl.clone(), &image);

        let font = Font::new(gl.clone());

        Self {
            width,
            height,
            pipeline,
            quad,
            title_bar,
            texture,
            windows: vec![],
            font,
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn draw(&self) {
        self.pipeline.program.bind();

        let view = Isometry3::look_at_rh(
            &Point3::new(0.0, 0.0, 0.5),
            &Point3::origin(),
            &Vector3::new(0.0, 1.0, 0.0),
        )
        .to_homogeneous();
        self.pipeline.set_view(&view);

        let proj = nalgebra::Orthographic3::new(
            0.0,
            self.width as f32,
            self.height as f32,
            0.0,
            0.125,
            1.0,
        )
        .to_homogeneous();
        self.pipeline.set_proj(&proj);

        for window in &self.windows {
            self.draw_window(window);
        }
    }

    fn draw_window(&self, window: &Window) {
        self.pipeline.set_sampler(0);

        self.texture.bind();

        // Title bar
        let title_height = self.font.tile_height + 6;
        let transform = Matrix4::identity()
            .append_nonuniform_scaling(&Vector3::new(window.width as f32, title_height as f32, 0.0))
            .append_translation(&Vector3::new(window.x as f32, window.y as f32, 0.1));
        self.pipeline.set_transform(&transform);

        self.pipeline.draw(&self.title_bar);

        // Background
        let transform = Matrix4::identity()
            .append_nonuniform_scaling(&Vector3::new(
                window.width as f32,
                window.height as f32,
                0.0,
            ))
            .append_translation(&Vector3::new(window.x as f32, window.y as f32, 0.0));
        self.pipeline.set_transform(&transform);

        self.pipeline.draw(&self.quad);

        self.font.texture.bind();

        // Draw window title
        for (i, c) in window.name.chars().enumerate() {
            let transform = Matrix4::identity()
                .append_nonuniform_scaling(&Vector3::new(
                    self.font.tile_width as f32,
                    self.font.tile_height as f32,
                    0.0,
                ))
                .append_translation(&Vector3::new(
                    window.x as f32 + 4.0 + (self.font.tile_width as usize * i) as f32,
                    window.y as f32 + 4.0,
                    0.2,
                ));
            self.pipeline.set_transform(&transform);

            let quad = self.font.get(c);
            self.pipeline.draw(&quad);
        }

        // Draw window text content
        let mut current_line_x = 0;
        let mut current_line_space_offset = 0;
        let mut offset_y = 0;
        let window_margin = 4;

        for (i, word) in window.text.split(" ").enumerate() {
            let word_len = self.font.tile_width * word.len() as u32;
            current_line_x += word_len;

            if current_line_x + current_line_space_offset > (window.width - window_margin * 2) {
                current_line_x = word_len;
                current_line_space_offset = 0;
                offset_y += self.font.tile_height;
            } else if i > 0 {
                current_line_space_offset += self.font.tile_width;
            }

            for (j, c) in word.chars().enumerate() {
                let current_char_x = current_line_space_offset
                    + (current_line_x - word_len)
                    + self.font.tile_width * j as u32;
                let translation_x = window.x + (window_margin + current_char_x) as i32;
                let translation_y = window.y + (window_margin + title_height + offset_y) as i32;

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

                let quad = self.font.get(c);
                self.pipeline.draw(&quad);
            }
        }
    }
}

pub struct Text {
    value: String,
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

pub struct Window {
    pub width: u32,
    pub height: u32,
    x: i32,
    y: i32,
    name: String,

    text: Text,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            x: 10,
            y: 10,
            name: String::from("Test window"),
            text: Text::from(
                "Content is actually drawn inside the window, even if it is a very long string!",
            ),
        }
    }
}

struct Font {
    texture: Texture,
    tile_width: u32,
    tile_height: u32,
    /// @todo Optimize this by using one position buffer
    /// and one UV buffer with UVs for all letters.
    quads: Vec<Primitive>,
}

impl Font {
    fn create_quads(
        gl: GL,
        image_width: u32,
        image_height: u32,
        tile_width: u32,
        tile_height: u32,
    ) -> Vec<Primitive> {
        let mut quads = vec![];

        let row_count = image_height / tile_height;
        let column_count = image_width / tile_width;

        let expected_column_count = 32;
        assert!(column_count >= expected_column_count);

        for i in 0..row_count {
            for j in 0..expected_column_count {
                // 4 UVs to modify
                let mut quad = Geometry::quad();

                // Bottom-left
                quad.vertices[0].uv = [
                    (j * tile_width) as f32 / image_width as f32,
                    (i * tile_height + tile_height) as f32 / image_height as f32,
                ];
                // Bottom-right
                quad.vertices[1].uv = [
                    (j * tile_width + tile_width) as f32 / image_width as f32,
                    (i * tile_height + tile_height) as f32 / image_height as f32,
                ];
                // Top-right
                quad.vertices[2].uv = [
                    (j * tile_width + tile_width) as f32 / image_width as f32,
                    (i * tile_height) as f32 / image_height as f32,
                ];
                // Top-left
                quad.vertices[3].uv = [
                    (j * tile_width) as f32 / image_width as f32,
                    (i * tile_height) as f32 / image_height as f32,
                ];

                quads.push(Primitive::new(gl.clone(), &quad));
            }
        }

        quads
    }

    pub fn new(gl: GL) -> Self {
        let data = include_bytes!("../res/font/spd.png");
        let image = Image::from_png(data);

        let texture = Texture::from_image(gl.clone(), &image);

        let tile_width = 8;
        let tile_height = 13;
        let quads = Font::create_quads(
            gl.clone(),
            image.width,
            image.height,
            tile_width,
            tile_height,
        );

        Self {
            texture,
            tile_width,
            tile_height,
            quads,
        }
    }

    pub fn get(&self, c: char) -> &Primitive {
        let index = (c as u8 + 53 as u8) as usize;
        if index < self.quads.len() {
            &self.quads[index]
        } else {
            &self.quads[0]
        }
    }
}
