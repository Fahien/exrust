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
fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> WebGlShader {
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

struct Program {
    gl: GL,
    program: WebGlProgram,
}

impl Program {
    fn new(gl: GL, vert_src: &str, frag_src: &str) -> Self {
        let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_src);
        let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_src);

        let program = link_program(&gl, vert_shader, frag_shader);

        Self { gl, program }
    }

    fn bind(&self) {
        self.gl.use_program(Some(&self.program));
    }

    fn get_attrib_loc(&self, name: &str) -> i32 {
        self.gl.get_attrib_location(&self.program, name)
    }

    fn get_uniform_loc(&self, name: &str) -> Option<WebGlUniformLocation> {
        self.gl.get_uniform_location(&self.program, name)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}

struct PointPipeline {
    program: Program,
    position_loc: i32,
    point_size_loc: i32,
    color_loc: Option<WebGlUniformLocation>,
}

impl PointPipeline {
    fn new(gl: &GL, vert_src: &str, frag_src: &str) -> Self {
        let program = Program::new(gl.clone(), vert_src, frag_src);
        program.bind();

        let position_loc = program.get_attrib_loc("position");
        let point_size_loc = program.get_attrib_loc("point_size");
        let color_loc = program.get_uniform_loc("color");

        Self {
            program,
            position_loc,
            point_size_loc,
            color_loc,
        }
    }
}

struct DefaultPipeline {
    program: Program,
    transform_loc: Option<WebGlUniformLocation>,
    normal_transform_loc: Option<WebGlUniformLocation>,
}

impl DefaultPipeline {
    fn new(gl: &GL, vert_src: &str, frag_src: &str) -> Self {
        let program = Program::new(gl.clone(), vert_src, frag_src);
        program.bind();

        let transform_loc = program.get_uniform_loc("transform");
        let normal_transform_loc = program.get_uniform_loc("normal_transform");

        Self {
            program,
            transform_loc,
            normal_transform_loc,
        }
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

        // Normal
        let normal_loc = self.program.get_attrib_loc("in_normal");

        let offset = 7 * std::mem::size_of::<f32>() as i32;
        self.program.gl.vertex_attrib_pointer_with_i32(
            normal_loc as u32,
            3,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program
            .gl
            .enable_vertex_attrib_array(normal_loc as u32);

        // Texture coordinates
        let uv_loc = self.program.get_attrib_loc("in_uv");
        let offset = 10 * std::mem::size_of::<f32>() as i32;
        self.program.gl.vertex_attrib_pointer_with_i32(
            uv_loc as u32,
            2,
            GL::FLOAT,
            false,
            stride,
            offset,
        );
        self.program.gl.enable_vertex_attrib_array(uv_loc as u32);
    }
}

#[repr(C)]
struct Vertex {
    position: [f32; 3], // xy
    color: [f32; 4],    // rgba
    normal: [f32; 3],
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
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
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
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 1.0],
            },
            // Right
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                uv: [0.0, 1.0],
            },
            // Back
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                uv: [0.0, 1.0],
            },
            // Left
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                uv: [0.0, 1.0],
            },
            // Top
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                uv: [0.0, 1.0],
            },
            // Bottom
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
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

struct Node {
    transform: Isometry3<f32>,
    primitive: Primitive,
}

impl Node {
    fn new(primitive: Primitive) -> Self {
        Self {
            transform: Isometry3::identity(),
            primitive,
        }
    }
}

#[wasm_bindgen]
pub struct Context {
    performance: web_sys::Performance,
    canvas: HtmlCanvasElement,
    gl: WebGlRenderingContext,
    view: Rc<RefCell<Isometry3<f32>>>,
    point_pipeline: PointPipeline,
    default_pipeline: DefaultPipeline,
    nodes: Vec<Node>,
    texture: Texture,
}

fn create_point_program(gl: &WebGlRenderingContext) -> PointPipeline {
    let vert_src = r#"
        attribute vec2 position;
        attribute float point_size;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            gl_PointSize = point_size;
        }
        "#;

    let frag_src = r#"
        precision mediump float;

        uniform vec4 color;

        void main() {
            gl_FragColor = color;
        }
        "#;

    PointPipeline::new(gl, vert_src, frag_src)
}

fn create_default_program(gl: &WebGlRenderingContext) -> DefaultPipeline {
    let vert_src = r#"
        attribute vec3 in_position;
        attribute vec4 in_color;
        attribute vec3 in_normal;
        attribute vec2 in_uv;

        varying vec3 position;
        varying vec4 color;
        varying vec3 normal;
        varying vec2 uv;

        uniform mat4 transform;
        uniform mat4 normal_transform;
        uniform mat4 view;
        uniform mat4 proj;

        void main() {
            uv = in_uv;
            vec4 pos4 = view * transform * vec4(in_position, 1.0);
            position = pos4.xyz;
            gl_Position = proj * pos4;
            normal = mat3(normal_transform) * normalize(in_normal);
            color = in_color;
        }
        "#;

    let frag_src = r#"
        precision mediump float;

        varying vec3 position;
        varying vec4 color;
        varying vec3 normal;
        varying vec2 uv;

        uniform sampler2D sampler;
        uniform vec3 light_color;
        uniform vec3 light_position;

        void main() {
            vec3 light_direction = light_position - position;
            float n_dot_l = max(
                dot(
                    normalize(light_direction),
                    normalize(normal)
                ),
                0.0
            );
            vec3 diffuse = light_color * vec3(color) * n_dot_l;
            vec3 ambient = light_color * vec3(color) * 0.1;
            gl_FragColor = vec4(diffuse + ambient, color.a) * texture2D(sampler, uv);
        }
        "#;

    DefaultPipeline::new(gl, vert_src, frag_src)
}

#[wasm_bindgen]
impl Context {
    pub fn new() -> Result<Context, JsValue> {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();

        let canvas = get_canvas()?;
        let gl = get_gl_context(&canvas)?;

        let point_pipeline = create_point_program(&gl);
        let default_pipeline = create_default_program(&gl);

        // OpenGL uses a right-handed coordinate system
        let view = Rc::new(RefCell::new(Isometry3::look_at_rh(
            &Point3::new(0.0, 0.0, 12.0),
            &Point3::origin(),
            &Vector3::y_axis(),
        )));

        let mut nodes = vec![];

        let num = 1;
        for i in -num..num {
            for j in -num..num {
                for k in -num..num {
                    let mut node = Node::new(Primitive::cube(&gl));
                    node.transform.append_translation_mut(&Translation3::new(
                        i as f32 * 1.5,
                        j as f32 * 1.5,
                        k as f32 * 1.5,
                    ));

                    nodes.push(node);
                }
            }
        }
        let texture = Texture::new(gl.clone());

        let ret = Context {
            performance,
            canvas,
            gl,
            view,
            point_pipeline,
            default_pipeline,
            nodes,
            texture,
        };

        let document = window.document().unwrap();
        ret.set_onmousemove(&document);
        ret.set_onwheel(&document);

        Ok(ret)
    }

    fn set_onmousemove(&self, document: &Document) {
        let view = self.view.clone();
        let callback = Box::new(move |e: web_sys::MouseEvent| {
            if e.shift_key() {
                // Check if left button is pressed
                if e.buttons() == 1 {
                    // Camera panning
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

    fn set_onwheel(&self, document: &Document) {
        let view = self.view.clone();
        let callback = Box::new(move |e: web_sys::WheelEvent| {
            let x = -e.delta_x() as f32 / 256.0;
            let y = -e.delta_y() as f32 / 256.0;
            // Camera zoom in/out
            view.borrow_mut()
                .append_translation_mut(&Translation3::new(x, 0.0, y));
        });
        let closure =
            wasm_bindgen::closure::Closure::wrap(callback as Box<dyn FnMut(web_sys::WheelEvent)>);
        document.set_onwheel(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    /// Draws a point at position x and y
    pub fn draw_point(&self, x: f32, y: f32) -> Result<(), JsValue> {
        self.point_pipeline.program.bind();

        self.gl
            .vertex_attrib1f(self.point_pipeline.point_size_loc as u32, 16.0);
        self.gl
            .vertex_attrib3f(self.point_pipeline.position_loc as u32, x, y, 0.0);
        self.gl
            .uniform4f(self.point_pipeline.color_loc.as_ref(), 0.0, 1.0, 0.0, 1.0);

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl.draw_arrays(GL::POINTS, 0, 1);

        Ok(())
    }

    /// Draws a primitive
    pub fn draw_primitive(&self) -> Result<(), JsValue> {
        self.gl.enable(GL::DEPTH_TEST);
        self.default_pipeline.program.bind();

        // View
        let view_loc = self.default_pipeline.program.get_uniform_loc("view");

        self.gl.uniform_matrix4fv_with_f32_array(
            view_loc.as_ref(),
            false,
            self.view.borrow().to_homogeneous().as_slice(),
        );

        // Proj
        let proj_loc = self.default_pipeline.program.get_uniform_loc("proj");

        let width = self.canvas.width() as f32;
        let height = self.canvas.height() as f32;
        let proj = nalgebra::Perspective3::new(width / height, 3.14 / 4.0, 0.125, 256.0);
        self.gl.uniform_matrix4fv_with_f32_array(
            proj_loc.as_ref(),
            false,
            proj.to_homogeneous().as_slice(),
        );

        // Lighting
        let light_color_loc = self.default_pipeline.program.get_uniform_loc("light_color");
        self.gl.uniform3f(light_color_loc.as_ref(), 1.0, 1.0, 1.0);

        let light_position_loc = self
            .default_pipeline
            .program
            .get_uniform_loc("light_position");
        self.gl
            .uniform3f(light_position_loc.as_ref(), 4.0, 1.0, 1.0);

        // Texture
        self.texture.bind();
        let sampler_loc = self.default_pipeline.program.get_uniform_loc("sampler");
        self.gl.uniform1i(sampler_loc.as_ref(), 0);

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        // Time
        let now = self.performance.now();

        // Draw all nodes
        for node in &self.nodes {
            node.primitive.bind(&self.gl);
            self.default_pipeline.bind_attribs();

            let mut transform = node.transform.clone();

            let rotation = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), now as f32 / 4096.0);
            transform.append_rotation_mut(&rotation);
            let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), now as f32 / 4096.0);
            transform.append_rotation_mut(&rotation);

            self.gl.uniform_matrix4fv_with_f32_array(
                self.default_pipeline.transform_loc.as_ref(),
                false,
                transform.to_homogeneous().as_slice(),
            );

            let normal_transform = transform.inverse().to_homogeneous().transpose();
            self.gl.uniform_matrix4fv_with_f32_array(
                self.default_pipeline.normal_transform_loc.as_ref(),
                false,
                normal_transform.as_slice(),
            );

            self.gl.draw_elements_with_i32(
                GL::TRIANGLES,
                node.primitive.index_count,
                GL::UNSIGNED_BYTE,
                0,
            );
        }

        Ok(())
    }
}
