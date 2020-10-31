mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Wrap web-sys console log function in a println! style macro
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

trait ToJsFloat32Array {
    /// Returns a TypedArray which is a view into this vector.
    /// Please do not reallocate memory while the view is alive or it can become invalid.
    unsafe fn to_js(&self) -> js_sys::Float32Array;
}

impl ToJsFloat32Array for Vec<f32> {
    unsafe fn to_js(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::view(self)
    }
}

/// Returns a WebGL Context
fn get_gl_context() -> Result<GL, JsValue> {
    utils::set_panic_hook();

    let doc = window().unwrap().document().unwrap();
    let canvas = doc.get_element_by_id("area").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    Ok(canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?)
}

/// Short WebGL program which simply clears a drawing area specified by a canvas tag
#[wasm_bindgen]
pub fn clear_drawing_area() -> Result<(), JsValue> {
    let gl = get_gl_context()?;

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

#[wasm_bindgen]
pub struct Context {
    gl: WebGlRenderingContext,
    point_program: WebGlProgram,
    triangle_program: WebGlProgram,
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
        attribute vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
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

#[wasm_bindgen]
impl Context {
    pub fn new() -> Result<Context, JsValue> {
        let gl = get_gl_context()?;
        let point_program = create_point_program(&gl);
        let triangle_program = create_triangle_program(&gl);

        Ok(Context {
            gl,
            point_program,
            triangle_program,
        })
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

    /// Draws a triangle
    pub fn draw_triangle(&self) -> Result<(), JsValue> {
        self.gl.use_program(Some(&self.triangle_program));

        let vertex_buffer = self.gl.create_buffer();
        self.gl
            .bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.as_ref());

        let vertices: Vec<f32> = vec![-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];

        self.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            unsafe { &vertices.to_js() },
            GL::STATIC_DRAW,
        );

        let position_loc = self
            .gl
            .get_attrib_location(&self.triangle_program, "position");

        self.gl
            .vertex_attrib_pointer_with_i32(position_loc as u32, 2, GL::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(position_loc as u32);

        let color_loc = self
            .gl
            .get_uniform_location(&self.triangle_program, "color");
        self.gl.uniform4f(color_loc.as_ref(), 0.0, 1.0, 0.0, 1.0);

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl.draw_arrays(GL::TRIANGLES, 0, 3);

        Ok(())
    }
}
