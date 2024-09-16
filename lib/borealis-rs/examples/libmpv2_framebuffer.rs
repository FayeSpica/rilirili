extern crate gl;
extern crate sdl2;

use gl::types::*;
use libmpv2_sys::{
    mpv_command_string, mpv_create, mpv_destroy, mpv_handle, mpv_initialize, mpv_opengl_fbo,
    mpv_opengl_init_params, mpv_render_context, mpv_render_context_create,
    mpv_render_context_render, mpv_render_context_report_swap, mpv_render_param,
    mpv_render_param_type_MPV_RENDER_PARAM_API_TYPE, mpv_render_param_type_MPV_RENDER_PARAM_FLIP_Y,
    mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
    mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_INIT_PARAMS, mpv_set_option_string,
    MPV_RENDER_API_TYPE_OPENGL,
};
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRGB, nvgRect, nvgReset,
    nvgResetTransform, nvgRestore, nvgSave, nvgScale,
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ffi::{c_float, c_int, c_void, CStr, CString};
use std::ptr;
use std::str;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    let window = video_subsystem
        .window("OpenGL Framebuffer Example", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Shader sources
    let vertex_shader_src = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;

        out vec2 TexCoord;

        void main() {
            gl_Position = vec4(aPos, 1.0);
            TexCoord = aTexCoord;
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core
        out vec4 FragColor;
        in vec2 TexCoord;
        uniform sampler2D screenTexture;

        void main() {
            FragColor = texture(screenTexture, TexCoord);
        }
    "#;

    // Create shaders and programs
    let shader_program = unsafe {
        let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER);
        let program = link_program(vertex_shader, fragment_shader);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    };

    // Set up vertices for a full-screen quad
    let vertices: [f32; 20] = [
        // (x, y, z) positions   // (x, y) texture coords
        0.5, 0.5, 0.0, 1.0, 1.0, 0.5, -0.5, 0.0, 1.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0, -0.5, 0.5,
        0.0, 0.0, 1.0,
    ];

    let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            &indices[0] as *const u32 as *const _,
            gl::STATIC_DRAW,
        );

        let stride = 5 * std::mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<GLfloat>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
    }

    // Framebuffer setup
    let mut media_framebuffer = 0;
    let mut media_texture = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut media_framebuffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, media_framebuffer);

        gl::GenTextures(1, &mut media_texture);
        gl::BindTexture(gl::TEXTURE_2D, media_texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            media_texture,
            0,
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    let mut mpv_gl_ctx: *mut mpv_render_context = ptr::null_mut();
    /// mpv
    unsafe {
        let mpv_handle = mpv_create();
        if mpv_handle.is_null() {
            eprintln!("Failed to create MPV instance");
            return;
        }

        // Initialize MPV
        if mpv_initialize(mpv_handle) < 0 {
            eprintln!("Failed to initialize MPV");
            mpv_destroy(mpv_handle);
            return;
        }

        let raw = CString::new(format!("{} {} {}", "loadfile", VIDEO_URL, "replace")).unwrap();

        // Send the command to MPV to load the file
        if mpv_command_string(mpv_handle, raw.as_ptr()) < 0 {
            eprintln!("Failed to load file");
            mpv_destroy(mpv_handle);
            return;
        }

        libmpv_set_option_string(mpv_handle, "ytdl", "no");
        libmpv_set_option_string(mpv_handle, "terminal", "yes");
        libmpv_set_option_string(mpv_handle, "msg-level", "all=v");
        libmpv_set_option_string(mpv_handle, "vo", "libmpv");
        libmpv_set_option_string(mpv_handle, "hwdec", "auto");

        // Create MPV render context for OpenGL
        let mut render_params: [mpv_render_param; 3] = [
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_API_TYPE,
                data: MPV_RENDER_API_TYPE_OPENGL.as_ptr() as *mut c_void,
            },
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_INIT_PARAMS,
                data: &mpv_opengl_init_params {
                    get_proc_address: Some(get_proc_address),
                    get_proc_address_ctx: Box::into_raw(Box::new(video_subsystem)) as *mut c_void,
                } as *const _ as *mut _,
            },
            mpv_render_param {
                type_: 0,
                data: ptr::null_mut(),
            },
        ];

        if mpv_render_context_create(&mut mpv_gl_ctx, mpv_handle, render_params.as_mut_ptr()) < 0 {
            eprintln!("Failed to create MPV render context");
            mpv_destroy(mpv_handle);
            return;
        }
    }

    let context = unsafe {
        nanovg_sys::gladLoadGL();
        let f = nanovg_sys::NVGcreateFlags::NVG_STENCIL_STROKES
            | nanovg_sys::NVGcreateFlags::NVG_ANTIALIAS;
        nanovg_sys::nvgCreateGL3(f.bits())
    };
    // Main loop
    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Bind to framebuffer and draw scene
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, SCREEN_WIDTH as GLsizei, SCREEN_HEIGHT as GLsizei);
            gl::ClearColor(0.0, 0.5, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            nvgBeginFrame(
                context,
                SCREEN_WIDTH as c_float,
                SCREEN_HEIGHT as c_float,
                1.5,
            );
            nvgScale(context, 1.5, 1.5);

            nvgSave(context);

            nvgBeginPath(context);
            nvgRect(context, 100.0, 100.0, 100.0, 100.0);
            nvgFillColor(context, nvgRGB(0, 255, 255));
            nvgFill(context);

            nvgRestore(context);

            nvgSave(context);

            gl::BindFramebuffer(gl::FRAMEBUFFER, media_framebuffer);
            gl::Viewport(0, 0, SCREEN_WIDTH as GLsizei, SCREEN_HEIGHT as GLsizei);
            gl::ClearColor(0.8, 0.0, 0.0, 0.5);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let mut render_params = [
                mpv_render_param {
                    type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
                    data: &mpv_opengl_fbo {
                        fbo: media_framebuffer as c_int,
                        w: SCREEN_WIDTH as c_int,
                        h: SCREEN_HEIGHT as c_int,
                        internal_format: 0,
                    } as *const _ as *mut _,
                },
                mpv_render_param {
                    type_: mpv_render_param_type_MPV_RENDER_PARAM_FLIP_Y,
                    data: &1 as *const _ as *mut _,
                },
                mpv_render_param {
                    type_: 0,
                    data: ptr::null_mut(),
                },
            ];

            mpv_render_context_render(mpv_gl_ctx, render_params.as_mut_ptr());

            // nvgRestore(context);

            // Bind to default framebuffer and draw quad
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, SCREEN_WIDTH as GLsizei, SCREEN_HEIGHT as GLsizei);

            gl::UseProgram(shader_program);
            gl::BindTexture(gl::TEXTURE_2D, media_texture);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            nvgRestore(context);

            // nvgBeginFrame(
            //     context,
            //     SCREEN_WIDTH as c_float,
            //     SCREEN_HEIGHT as c_float,
            //     1.5,
            // );
            //
            // nvgBeginPath(context);
            // nvgRect(context, 400.0, 400.0, 100.0, 100.0);
            // nvgFillColor(context, nvgRGB(255, 0, 255));
            // nvgFill(context);

            nvgResetTransform(context);
            nvgEndFrame(context);

            window.gl_swap_window();
        }
    }
}

unsafe fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
    let shader = gl::CreateShader(shader_type);
    let c_str = CString::new(src.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    let mut success = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer = vec![0; len as usize];
        gl::GetShaderInfoLog(
            shader,
            len,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut GLchar,
        );
        panic!(
            "{}",
            str::from_utf8(&buffer).expect("ShaderInfoLog not valid utf8")
        );
    }

    shader
}

unsafe fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);
    gl::LinkProgram(program);

    let mut success = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        let mut len = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer = vec![0; len as usize];
        gl::GetProgramInfoLog(
            program,
            len,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut GLchar,
        );
        panic!(
            "{}",
            str::from_utf8(&buffer).expect("ProgramInfoLog not valid utf8")
        );
    }

    program
}

fn libmpv_set_option_string(mpv_handle: *mut mpv_handle, name: &str, data: &str) {
    let name = CString::new(name).unwrap();
    let data = CString::new(data).unwrap();
    unsafe {
        mpv_set_option_string(mpv_handle, name.as_ptr(), data.as_ptr());
    }
}

const VIDEO_URL: &str = "test-data/test-video.mp4";

unsafe extern "C" fn get_proc_address(ctx: *mut c_void, name: *const i8) -> *mut c_void {
    let cname = CStr::from_ptr(name);
    let sdl_video_subsystem = &*(ctx as *mut sdl2::VideoSubsystem);
    let fn_name = cname.to_str().unwrap();
    sdl_video_subsystem.gl_get_proc_address(fn_name) as *mut _
}
