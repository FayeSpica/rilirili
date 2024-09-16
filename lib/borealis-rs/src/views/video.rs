use std::collections::HashMap;
use std::{env, ptr};
use std::ffi::{c_float, c_int, c_void, CStr, CString};
use gl::{COLOR_BUFFER_BIT, FRAMEBUFFER, types};
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};
use libmpv2_sys::*;
use nanovg_sys::{nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRect, nvgReset, nvgRestore, nvgRGB, nvgRGBA, nvgSave};
use sdl2::{EventSubsystem, VideoSubsystem};
use sdl2::video::GLContext;
use crate::core::frame_context::FrameContext;
use crate::core::global::{content_height, content_width, window_height, window_width};
use crate::core::theme::theme;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{Axis, BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct GLShader {
    prog: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    vao: GLuint,
}

pub struct Video {
    box_view_data: BoxViewData,
    mpv_render_context: *mut mpv_render_context,
    default_framebuffer: GLuint,
    media_framebuffer: GLuint,
    media_texture: GLuint,
    shader: GLShader,
    mpv_handle: *mut mpv_handle,
    vertices: [f32; 20],
    mpv_opengl_fbo: mpv_opengl_fbo,
    mpv_params: [mpv_render_param; 3]
}


unsafe extern "C" fn get_proc_address(ctx: *mut c_void, name: *const i8) -> *mut c_void {
    let cname = CStr::from_ptr(name);
    let sdl_video_subsystem =  &*(ctx as *mut sdl2::VideoSubsystem);
    let fn_name = cname.to_str().unwrap();
    sdl_video_subsystem.gl_get_proc_address(fn_name) as *mut _
}


fn libmpv_set_option_string(mpv_handle: *mut mpv_handle, name: &str, data: &str) {
    let name = CString::new(name).unwrap();
    let data = CString::new(data).unwrap();
    unsafe {
        mpv_set_option_string(mpv_handle, name.as_ptr(), data.as_ptr());
    }
}

// Function to check for shader compile errors
fn check_shader_compile_errors(shader: GLuint) {
    let mut success: GLint = 0;
    let mut info_log = vec![0; 1024];
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(
                shader,
                info_log.len() as GLint,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "ERROR::SHADER::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap_or_default()
            );
        }
    }
}

// Function to check for shader program linking errors
fn check_program_link_errors(program: GLuint) {
    let mut success: GLint = 0;
    let mut info_log = vec![0; 1024];
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            gl::GetProgramInfoLog(
                program,
                info_log.len() as GLint,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "ERROR::PROGRAM::LINKING_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap_or_default()
            );
        }
    }
}

const VIDEO_URL: &str = "./test-data/test-video.mp4";

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

void main() {
    gl_Position = vec4(aPos, 1.0);
    TexCoord = aTexCoord;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;
in vec2 TexCoord;
uniform sampler2D screenTexture;

void main() {
    FragColor = texture(screenTexture, TexCoord);
}
"#;

#[derive(Debug)]
enum UserEvent {
    MpvEventAvailable,
    RedrawRequested,
}

impl Video {
    pub fn new(x: f32, y: f32, width: f32, height: f32, video_subsystem: VideoSubsystem) -> Self {
        let mut mpv_render_context: *mut mpv_render_context = ptr::null_mut();
        let mut default_framebuffer: GLuint = GLuint::default();
        let mut media_framebuffer: GLuint = GLuint::default();
        let mut media_texture: GLuint = GLuint::default();
        let mut shader_program: GLuint = GLuint::default();
        let mut shader: GLShader = GLShader {
            prog: 0,
            vbo: 0,
            ebo: 0,
            vao: 0,
        };
        let mut vertices: [f32; 20] = [
            // (x, y, z) positions   // (x, y) texture coords
            0.5,  0.5,  0.0,  1.0,  1.0,
            0.5, -0.5,  0.0,  1.0,  0.0,
            -0.5, -0.5,  0.0,  0.0,  0.0,
            -0.5,  0.5,  0.0,  0.0,  1.0,
        ];
        let mpv_handle = unsafe {
            /// vbo vao
            // build and compile our shader program
            // ------------------------------------
            // vertex shader
            let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
            // fragment shader
            let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);

            // link shaders
            shader_program = link_program(vertex_shader, fragment_shader);

            // Clean up shaders
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader.prog = shader_program;


            // set up vertex data (and buffer(s)) and configure vertex attributes
            // ------------------------------------------------------------------
            let indices: [u32; 6] = [
                0, 1, 3,
                1, 2, 3,
            ];


            gl::GenVertexArrays(1, &mut shader.vao);
            gl::GenBuffers(1, &mut shader.vbo);
            gl::GenBuffers(1, &mut shader.ebo);

            gl::BindVertexArray(shader.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, shader.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, shader.ebo);
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

            /// opengl framebuffer
            // create frame buffer
            gl::GenFramebuffers(1, &mut media_framebuffer);
            gl::BindFramebuffer(FRAMEBUFFER, media_framebuffer);

            // create texture
            gl::GenTextures(1, &mut media_texture);
            gl::BindTexture(gl::TEXTURE_2D, media_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                window_width() as GLsizei,
                window_height() as GLsizei,
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
                media_texture,
                0,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);


            info!("create fbo and texture done");

            /// init mpv

            let mpv_handle = mpv_create();
            if mpv_handle.is_null() {
                eprintln!("Failed to create MPV instance");
                panic!("Failed to create MPV instance");
            }

            // Initialize MPV
            if mpv_initialize(mpv_handle) < 0 {
                eprintln!("Failed to initialize MPV");
                mpv_destroy(mpv_handle);
                panic!("Failed to initialize MPV");
            }

            let raw = CString::new(format!("{} {} {}", "loadfile", VIDEO_URL, "replace")).unwrap();

            // Send the command to MPV to load the file
            if mpv_command_string(mpv_handle, raw.as_ptr()) < 0 {
                eprintln!("Failed to load file");
                mpv_destroy(mpv_handle);
                panic!("Failed to load file");
            }

            libmpv_set_option_string(mpv_handle, "ytdl", "no");
            libmpv_set_option_string(mpv_handle, "audio-channels", "stereo");
            libmpv_set_option_string(mpv_handle, "idle", "yes");
            libmpv_set_option_string(mpv_handle, "loop-file", "no");
            libmpv_set_option_string(mpv_handle, "osd-level", "0");
            libmpv_set_option_string(mpv_handle, "video-timing-offset", "0");
            libmpv_set_option_string(mpv_handle, "keep-open", "yes");
            libmpv_set_option_string(mpv_handle, "hr-seek", "yes");
            libmpv_set_option_string(mpv_handle, "reset-on-next-file", "speed,pause");
            libmpv_set_option_string(mpv_handle, "terminal", "yes");
            libmpv_set_option_string(mpv_handle, "msg-level", "all=v");
            libmpv_set_option_string(mpv_handle, "vo", "libmpv");
            libmpv_set_option_string(mpv_handle, "pulse-latency-hacks", "no");
            libmpv_set_option_string(mpv_handle, "ytdl", "no");
            libmpv_set_option_string(mpv_handle, "terminal", "no");
            // libmpv_set_option_string(mpv_handle, "msg-level", "all=v");
            libmpv_set_option_string(mpv_handle, "vo", "libmpv");


            // libmpv_set_option_string(mpv_handle, "hwdec", "auto");

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

            let status = mpv_render_context_create(&mut mpv_render_context, mpv_handle, render_params.as_mut_ptr());
            info!("status: {}", status);
            if status < 0 {
                eprintln!("Failed to create MPV render context");
                mpv_destroy(mpv_handle);
                panic!("Failed to create MPV render context");
            }

            mpv_handle
        };

        let s = Self {
            box_view_data: BoxViewData {
                view_data: Default::default(),
                axis: Axis::Row,
                children: vec![],
                default_focused_index: 0,
                last_focused_view: None,
                forwarded_attributes: Default::default(),
                box_view: None,
            },
            mpv_render_context,
            default_framebuffer,
            media_framebuffer,
            media_texture,
            shader,
            mpv_handle,
            vertices,
            mpv_opengl_fbo: mpv_opengl_fbo {
                fbo: media_framebuffer as c_int,
                w: window_width() as c_int,
                h: window_height() as c_int,
                internal_format: 0,
            },
            mpv_params: [
                mpv_render_param {
                    type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
                    data: &mpv_opengl_fbo {
                        fbo: media_framebuffer as c_int,
                        w: window_width() as c_int,
                        h: window_height() as c_int,
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
            ],
        };
        s.set_width(width);
        s.set_height(height);
        s.set_position_top(x);
        s.set_position_left(y);
        s
    }
}

type DeleterFn = unsafe fn(*mut c_void);

unsafe fn free_void_data<T>(ptr: *mut c_void) {
    drop(Box::<T>::from_raw(ptr as *mut T));
}

pub trait VideoTrait: BoxTrait {

    fn video_data(&self) -> &Video;
    fn video_data_mut(&mut self) -> &mut Video;

    fn set_frame_size(&mut self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        let content_height = content_height();
        let content_width = content_width();
        unsafe {
            let draw_width = width * ctx.pixel_ratio;
            let draw_height = height * ctx.pixel_ratio;

            // info!("MPVCore::setFrameSize: {}/{}", draw_width, draw_height);
            gl::BindTexture(gl::TEXTURE_2D, self.video_data().media_texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, draw_width as GLsizei, draw_height as GLsizei, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr::null());
            self.video_data_mut().mpv_opengl_fbo.w = draw_width as c_int;
            self.video_data_mut().mpv_opengl_fbo.h = draw_height as c_int;

            let new_min_x = x / content_width * 2.0 - 1.0;
            let new_min_y = 1.0 - y / content_height * 2.0;
            let new_max_x = (x + width) / content_width * 2.0 - 1.0;
            let new_max_y = 1.0 - (y + height) / content_height * 2.0;

            self.video_data_mut().vertices[0] = new_max_x;
            self.video_data_mut().vertices[1] = new_min_y;
            self.video_data_mut().vertices[5] = new_max_x;
            self.video_data_mut().vertices[6] = new_max_y;
            self.video_data_mut().vertices[10] = new_min_x;
            self.video_data_mut().vertices[11] = new_max_y;
            self.video_data_mut().vertices[15] = new_min_x;
            self.video_data_mut().vertices[16] = new_min_y;

            gl::BindBuffer(gl::ARRAY_BUFFER, self.video_data().shader.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.video_data().vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,  // 数据大小
                self.video_data().vertices.as_ptr() as *const c_void,                          // 数据指针
                gl::STATIC_DRAW,
            );

            self.video_data_mut().mpv_params[0].type_ = mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO;
            self.video_data_mut().mpv_params[0].data = &self.video_data().media_framebuffer as *const _ as *mut _;

            // mpv_render_context_render(self.video_data().mpv_render_context, self.video_data_mut().mpv_params.as_mut_ptr());
            // gl::BindFramebuffer(gl::FRAMEBUFFER, self.video_data().default_framebuffer);
            // gl::Viewport(0, 0, window_width() as GLsizei, window_height() as GLsizei);
            // mpv_render_context_report_swap(self.video_data().mpv_render_context);
        }
    }

    fn draw(&mut self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        // info!("video draw {} {}", width, height);
        let content_height = content_height();
        let content_width = content_width();
        self.set_frame_size(ctx, x, y, width, height);
        unsafe {
            // nvgEndFrame(ctx.context);
            nvgSave(ctx.context);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.video_data().media_framebuffer);
            gl::Viewport(0, 0, window_width() as GLsizei, window_height() as GLsizei);
            gl::ClearColor(0.2, 0.0, 0.0, 0.5);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let mut mpv_params = [
                mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
                data: &mpv_opengl_fbo {
                    fbo: self.video_data().media_framebuffer as c_int,
                    w: window_width() as c_int,
                    h: window_height() as c_int,
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

            // 绘制视频
            mpv_render_context_render(self.video_data().mpv_render_context, mpv_params.as_mut_ptr());

            // Bind to default framebuffer and draw quad
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            // gl::ClearColor(0.0, 0.5, 0.0, 1.0);
            // gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.video_data().shader.prog);
            gl::BindTexture(gl::TEXTURE_2D, self.video_data().media_texture);
            gl::BindVertexArray(self.video_data().shader.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            nvgRestore(ctx.context);

            // info!("({}, {}) ({}, {}) ({}, {})", x, y, width, height, content_width, content_height);
            // nvgBeginFrame(ctx.context, content_width, content_width, 1.5);
            // nvgBeginPath(ctx.context);
            // nvgFillColor(ctx.context, theme("brls/background"));
            // nvgRect(ctx.context, 0.0, 0.0, content_width, content_height);
            // nvgRect(ctx.context, 0.0, 0.0, x, content_height);
            // nvgRect(ctx.context, x + width, 0.0, content_width - x - width, content_height);
            // nvgRect(ctx.context, x - 1.0, 0.0, width + 2.0 , y);
            // nvgRect(ctx.context, x - 1.0, y + height, width + 2.0, content_height - y - height);
            // nvgFillColor(ctx.context, nvgRGBA(255, 0, 0, 150));
            // nvgFill(ctx.context);
            //
            // nvgEndFrame(ctx.context);
        }
    }
}

impl BoxTrait for Video {}

impl ViewDrawer for Video {

}

impl ViewLayout for Video {}

impl ViewStyle for Video {}

impl ViewBase for Video {
    fn data(&self) -> &ViewData {
        &self.box_view_data.view_data
    }

    fn data_mut(&mut self) -> &mut ViewData {
        &mut self.box_view_data.view_data
    }
}

impl VideoTrait for Video {
    fn video_data(&self) -> &Video {
        self
    }

    fn video_data_mut(&mut self) -> &mut Video {
        self
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
            std::str::from_utf8(&buffer)
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
            std::str::from_utf8(&buffer)
                .expect("ProgramInfoLog not valid utf8")
        );
    }

    program
}