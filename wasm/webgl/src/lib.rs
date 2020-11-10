mod utils;

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use nalgebra::{Isometry3, Point3, Translation3, UnitQuaternion, Vector3};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Wrap web-sys console log function in a println! style macro
macro_rules! log {
    ( $( $t:tt )* ) => {
        log(&format!( $( $t )* ));
    }
}

trait ToJsArray {
    /// Returns a TypedArray which is a view into this vector.
    /// Please do not reallocate memory while the view is alive or it can become invalid.
    unsafe fn to_js(&self) -> js_sys::Float32Array;
}

impl ToJsArray for Vec<Vertex> {
    unsafe fn to_js(&self) -> js_sys::Float32Array {
        let len = self.len() * std::mem::size_of::<Vertex>() / std::mem::size_of::<f32>();
        let floats = std::slice::from_raw_parts(self.as_ptr() as *const f32, len);
        log!("Vertices: {:?}", floats);
        js_sys::Float32Array::view(floats)
    }
}

/// Returns a WebGL Context
fn get_canvas() -> Result<HtmlCanvasElement, JsValue> {
    utils::set_panic_hook();

    let doc = window().unwrap().document().unwrap();
    let canvas = doc.get_element_by_id("area").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);

    Ok(canvas)
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<GL, JsValue> {
    Ok(canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?)
}

/// Short WebGL program which simply clears a drawing area specified by a canvas tag
#[wasm_bindgen]
pub fn clear_drawing_area() -> Result<(), JsValue> {
    let canvas = get_canvas().unwrap();
    let gl = get_gl_context(&canvas)?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(GL::COLOR_BUFFER_BIT);

    Ok(())
}

/// Compiles source code into a shader object
pub fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> WebGlShader {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if !gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap()
    {
        let msg = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error"));
        panic!("Failed to compile shader: {}", msg);
    }

    shader
}

/// Links vertex and fragment shader into a shader program
fn link_program(gl: &GL, vert: WebGlShader, frag: WebGlShader) -> WebGlProgram {
    let program = gl.create_program().unwrap();

    gl.attach_shader(&program, &vert);
    gl.attach_shader(&program, &frag);
    gl.link_program(&program);

    if !gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap()
    {
        let msg = gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error"));
        panic!("Failed to link program: {}", msg);
    }

    gl.delete_shader(Some(&vert));
    gl.delete_shader(Some(&frag));

    program
}

#[repr(C)]
struct Vertex {
    position: [f32; 3], // xy
    color: [f32; 4],    // rgba
    uv: [f32; 2],
}

/// Generic primitive geometry
struct Primitive {
    vertex_buffer: Option<WebGlBuffer>,
    index_buffer: Option<WebGlBuffer>,
    index_count: i32,
}

impl Primitive {
    fn triangle(gl: &GL) -> Self {
        let vertex_buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.as_ref());

        let vertices: Vec<Vertex> = vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.5, 1.0],
            },
        ];

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            unsafe { &vertices.to_js() },
            GL::STATIC_DRAW,
        );

        let index_buffer = gl.create_buffer();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, index_buffer.as_ref());
        gl.buffer_data_with_u8_array(GL::ELEMENT_ARRAY_BUFFER, &[0, 1, 2], GL::STATIC_DRAW);

        Self {
            vertex_buffer,
            index_buffer,
            index_count: 3,
        }
    }

    fn cube(gl: &GL) -> Self {
        let vertex_buffer = gl.create_buffer();
        gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.as_ref());

        let vertices = vec![
            // Front
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
            },
            // Right
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
            },
            // Back
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
            },
            // Lef
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
            },
            // Top
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
            },
            // Bottom
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
            },
        ];

        let indices: Vec<u8> = vec![
            0, 1, 2, 0, 2, 3, // front face
            4, 5, 6, 4, 6, 7, // right
            8, 9, 10, 8, 10, 11, // back
            12, 13, 14, 12, 14, 15, // left
            16, 17, 18, 16, 18, 19, // top
            20, 21, 22, 20, 22, 23, // bottom
        ];

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            unsafe { &vertices.to_js() },
            GL::STATIC_DRAW,
        );

        let index_buffer = gl.create_buffer();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, index_buffer.as_ref());
        gl.buffer_data_with_u8_array(GL::ELEMENT_ARRAY_BUFFER, &indices, GL::STATIC_DRAW);

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as i32,
        }
    }

    fn bind(&self, gl: &GL) {
        gl.bind_buffer(GL::ARRAY_BUFFER, self.vertex_buffer.as_ref());
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, self.index_buffer.as_ref());
    }
}

struct Texture {
    gl: GL,
    handle: WebGlTexture,
}

impl Texture {
    fn new(gl: GL) -> Self {
        let handle = gl.create_texture().expect("Failed to create texture");

        let texture = Self { gl, handle };

        texture.bind();

        texture
            .gl
            .tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        texture
            .gl
            .tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

        let pixels = [
            255u8, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
        ];
        texture.upload(2, 2, &pixels);

        texture
    }

    fn bind(&self) {
        self.gl.active_texture(GL::TEXTURE0);
        self.gl.bind_texture(GL::TEXTURE_2D, Some(&self.handle));
    }

    /// Uploads pixels data to the texture memory in the GPU
    fn upload(&self, width: u32, height: u32, pixels: &[u8]) {
        self.gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                GL::TEXTURE_2D,
                0,
                GL::RGBA as i32,
                width as i32,
                height as i32,
                0,
                GL::RGBA,
                GL::UNSIGNED_BYTE,
                Some(&pixels),
            )
            .expect("Failed to upload texture data");
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.gl.delete_texture(Some(&self.handle))
    }
}

#[wasm_bindgen]
pub struct Context {
    performance: web_sys::Performance,
    canvas: HtmlCanvasElement,
    gl: WebGlRenderingContext,
    view: Rc<RefCell<Isometry3<f32>>>,
    point_program: WebGlProgram,
    triangle_program: WebGlProgram,
    primitive: Primitive,
    texture: Texture,
}

fn create_point_program(gl: &WebGlRenderingContext) -> WebGlProgram {
    let vert_source = r#"
        attribute vec2 position;
        attribute float point_size;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            gl_PointSize = point_size;
        }
        "#;

    let frag_source = r#"
        precision mediump float;

        uniform vec4 color;

        void main() {
            gl_FragColor = color;
        }
        "#;

    let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, vert_source);
    let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, frag_source);

    link_program(gl, vert_shader, frag_shader)
}

fn create_triangle_program(gl: &WebGlRenderingContext) -> WebGlProgram {
    let vert_source = r#"
        attribute vec3 in_position;
        attribute vec4 in_color;
        attribute vec2 in_uv;

        varying vec4 color;
        varying vec2 uv;

        uniform mat4 transform;
        uniform mat4 view;
        uniform mat4 proj;

        void main() {
            color = in_color;
            uv = in_uv;
            gl_Position = proj * view * transform * vec4(in_position, 1.0);
        }
        "#;

    let frag_source = r#"
        precision mediump float;

        varying vec4 color;
        varying vec2 uv;

        uniform sampler2D sampler;

        void main() {
            gl_FragColor = color * texture2D(sampler, uv);
        }
        "#;

    let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, vert_source);
    let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, frag_source);

    link_program(gl, vert_shader, frag_shader)
}

#[wasm_bindgen]
impl Context {
    pub fn new() -> Result<Context, JsValue> {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();

        let canvas = get_canvas()?;
        let gl = get_gl_context(&canvas)?;

        let point_program = create_point_program(&gl);
        let triangle_program = create_triangle_program(&gl);

        // OpenGL uses a right-handed coordinate system
        let view = Rc::new(RefCell::new(Isometry3::look_at_rh(
            &Point3::new(0.0, 0.0, 3.0),
            &Point3::origin(),
            &Vector3::y_axis(),
        )));

        let primitive = Primitive::cube(&gl);
        let texture = Texture::new(gl.clone());

        let ret = Context {
            performance,
            canvas,
            gl,
            view,
            point_program,
            triangle_program,
            primitive,
            texture,
        };

        ret.init();

        Ok(ret)
    }

    fn init(&self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let view = self.view.clone();
        let callback = Box::new(move |e: web_sys::MouseEvent| {
            if e.shift_key() {
                // Check if left button is pressed
                if e.buttons() == 1 {
                    let x = e.movement_x() as f32 / 256.0;
                    let y = -(e.movement_y() as f32 / 256.0);
                    view.borrow_mut()
                        .append_translation_mut(&Translation3::new(x, y, 0.0));
                }
            }
        });
        let closure =
            wasm_bindgen::closure::Closure::wrap(callback as Box<dyn FnMut(web_sys::MouseEvent)>);
        document.set_onmousemove(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    /// Draws a point at position x and y
    pub fn draw_point(&self, x: f32, y: f32) -> Result<(), JsValue> {
        self.gl.use_program(Some(&self.point_program));

        let position_loc = self.gl.get_attrib_location(&self.point_program, "position");
        self.gl.vertex_attrib3f(position_loc as u32, x, y, 0.0);

        let point_size_loc = self
            .gl
            .get_attrib_location(&self.point_program, "point_size");
        self.gl.vertex_attrib1f(point_size_loc as u32, 16.0);

        let color_loc = self.gl.get_uniform_location(&self.point_program, "color");
        self.gl.uniform4f(color_loc.as_ref(), 0.0, 1.0, 0.0, 1.0);

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl.draw_arrays(GL::POINTS, 0, 1);

        Ok(())
    }

    /// Draws a primitive
    pub fn draw_primitive(&self) -> Result<(), JsValue> {
        self.gl.enable(GL::DEPTH_TEST);
        self.gl.use_program(Some(&self.triangle_program));

        self.primitive.bind(&self.gl);

        // Position
        let position_loc = self
            .gl
            .get_attrib_location(&self.triangle_program, "in_position");

        // Number of bytes between each vertex element
        let stride = std::mem::size_of::<Vertex>() as i32;
        // Offset of vertex data from the beginning of the buffer
        let offset = 0;

        self.gl.vertex_attrib_pointer_with_i32(
            position_loc as u32,
            3,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.gl.enable_vertex_attrib_array(position_loc as u32);

        // Color
        let color_loc = self
            .gl
            .get_attrib_location(&self.triangle_program, "in_color");

        let offset = 3 * std::mem::size_of::<f32>() as i32;
        self.gl.vertex_attrib_pointer_with_i32(
            color_loc as u32,
            4,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.gl.enable_vertex_attrib_array(color_loc as u32);

        // Texture coordinates
        let uv_loc = self.gl.get_attrib_location(&self.triangle_program, "in_uv");
        let offset = 7 * std::mem::size_of::<f32>() as i32;
        self.gl
            .vertex_attrib_pointer_with_i32(uv_loc as u32, 2, GL::FLOAT, false, stride, offset);
        self.gl.enable_vertex_attrib_array(uv_loc as u32);

        // Transform
        let transform_loc = self
            .gl
            .get_uniform_location(&self.triangle_program, "transform");
        let mut transform = Isometry3::identity();
        transform.append_translation_mut(&Translation3::new(0.1, -0.1, 0.0));

        let now = self.performance.now();
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), now as f32 / 4096.0);
        transform.append_rotation_mut(&rotation);
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), now as f32 / 4096.0);
        transform.append_rotation_mut(&rotation);

        self.gl.uniform_matrix4fv_with_f32_array(
            transform_loc.as_ref(),
            false,
            transform.to_homogeneous().as_slice(),
        );

        // View
        let view_loc = self.gl.get_uniform_location(&self.triangle_program, "view");

        self.gl.uniform_matrix4fv_with_f32_array(
            view_loc.as_ref(),
            false,
            self.view.borrow().to_homogeneous().as_slice(),
        );

        // Proj
        let proj_loc = self.gl.get_uniform_location(&self.triangle_program, "proj");

        let width = self.canvas.width() as f32;
        let height = self.canvas.height() as f32;
        let proj = nalgebra::Perspective3::new(width / height, 3.14 / 4.0, 0.125, 8.0);
        self.gl.uniform_matrix4fv_with_f32_array(
            proj_loc.as_ref(),
            false,
            proj.to_homogeneous().as_slice(),
        );

        // Texture
        self.texture.bind();
        let sampler_loc = self
            .gl
            .get_uniform_location(&self.triangle_program, "sampler");
        self.gl.uniform1i(sampler_loc.as_ref(), 0);

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl.draw_elements_with_i32(
            GL::TRIANGLES,
            self.primitive.index_count,
            GL::UNSIGNED_BYTE,
            0,
        );

        Ok(())
    }
}
