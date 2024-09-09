use crate::core::theme::theme;
use crate::core::{
    create_shader, get_gl_string, gl, FRAGMENT_SHADER_SOURCE, VERTEX_DATA, VERTEX_SHADER_SOURCE,
};
use glutin::display::Display;
use glutin::prelude::GlDisplay;
use nanovg::Context;
use std::ffi::CString;

pub struct FrameContext {
    pub context: Context,
    pub pixel_ratio: f32,
    pub gl: gl::Gl,
}

impl FrameContext {
    pub fn new(gl_display: &Display) -> Self {
        trace!("FrameContext::new()");
        let gl = gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });
        let clear_color = theme("brls/clear");
        // OpenGL 设置
        unsafe {
            gl.Viewport(0, 0, 1920, 1080); // 设置视口
            gl.ClearColor(
                clear_color.rgba[0],
                clear_color.rgba[1],
                clear_color.rgba[2],
                clear_color.rgba[3],
            ); // 设置背景色
        }
        // 初始化 NanoVG
        let context = nanovg::ContextBuilder::new()
            .stencil_strokes()
            .antialias()
            .build()
            .expect("glfw: unable to init nanovg");

        unsafe {
            if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
                warn!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(&gl, gl::VERSION) {
                warn!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
                warn!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            // let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            // let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);
            //
            // let program = gl.CreateProgram();
            //
            // gl.AttachShader(program, vertex_shader);
            // gl.AttachShader(program, fragment_shader);
            //
            // gl.LinkProgram(program);
            //
            // gl.UseProgram(program);
            //
            // gl.DeleteShader(vertex_shader);
            // gl.DeleteShader(fragment_shader);
            //
            // let mut vao = std::mem::zeroed();
            // gl.GenVertexArrays(1, &mut vao);
            // gl.BindVertexArray(vao);
            //
            // let mut vbo = std::mem::zeroed();
            // gl.GenBuffers(1, &mut vbo);
            // gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            // gl.BufferData(
            //     gl::ARRAY_BUFFER,
            //     (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            //     VERTEX_DATA.as_ptr() as *const _,
            //     gl::STATIC_DRAW,
            // );
        }

        Self {
            context,
            pixel_ratio: 1.0,
            gl,
        }
    }

    pub fn vg(&self) -> &Context {
        &self.context
    }
}
