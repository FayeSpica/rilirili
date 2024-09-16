extern crate sdl2;
extern crate gl;

use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ffi::CString;
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
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

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

    let framebuffer_shader_src = r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(0.3, 0.3, 0.0, 1.0);
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

    // let framebuffer_program = unsafe {
    //     let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
    //     let fragment_shader = compile_shader(framebuffer_shader_src, gl::FRAGMENT_SHADER);
    //     let program = link_program(vertex_shader, fragment_shader);
    //
    //     gl::DeleteShader(vertex_shader);
    //     gl::DeleteShader(fragment_shader);
    //
    //     program
    // };

    // Set up vertices for a full-screen quad
    let vertices: [f32; 20] = [
        // positions   // texture coords
         0.5,  0.5,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.0,  0.0,  1.0,
    ];

    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3,
    ];

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
    let mut framebuffer = 0;
    let mut texture_colorbuffer = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut framebuffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

        gl::GenTextures(1, &mut texture_colorbuffer);
        gl::BindTexture(gl::TEXTURE_2D, texture_colorbuffer);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture_colorbuffer,
            0,
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

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
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            gl::ClearColor(0.2, 0.0, 0.0, 0.5);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // gl::UseProgram(framebuffer_program);
            // gl::BindVertexArray(vao);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            // Bind to default framebuffer and draw quad
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(0.0, 0.5, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindTexture(gl::TEXTURE_2D, texture_colorbuffer);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());


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
            str::from_utf8(&buffer)
                .expect("ShaderInfoLog not valid utf8")
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
            str::from_utf8(&buffer)
                .expect("ProgramInfoLog not valid utf8")
        );
    }

    program
}
