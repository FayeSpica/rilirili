use std::collections::HashMap;
use std::{env, ptr};
use std::ffi::{c_float, c_int, c_void, CStr, CString};
use gl::{COLOR_BUFFER_BIT, FRAMEBUFFER, types};
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint};
use libmpv2_sys::*;
use nanovg_sys::{nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRect, nvgReset, nvgRestore, nvgRGB, nvgSave};
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

pub struct Video {
    box_view_data: BoxViewData,
    mpv_render_context: *mut mpv_render_context,
    framebuffer: GLuint,
    media_texture: GLuint,
    shader_program: GLuint,
    mpv_handle: *mut mpv_handle,
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

fn create_shader(_type: GLenum, shader_src: &str) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(_type);
        let shader_cstr = CString::new(shader_src).unwrap();
        gl::ShaderSource(shader, 1, &shader_cstr.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // check for shader compile errors
        check_shader_compile_errors(shader);
        shader
    }
}

fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        // Link shaders into a program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        check_program_link_errors(shader_program);
        shader_program
    }
}

const VIDEO_URL: &str = "./test-data/test-video.mp4";

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
in vec3 aPos;
in vec2 aTexCoord;
out vec2 TexCoord;
void main()
{
    gl_Position = vec4(aPos, 1.0);
    TexCoord = aTexCoord;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
in vec2 TexCoord;
out vec4 FragColor;
uniform sampler2D ourTexture;
uniform float Alpha;
void main()
{
    FragColor = texture(ourTexture, TexCoord);
    FragColor.a = Alpha;
}
"#;

#[derive(Debug)]
enum UserEvent {
    MpvEventAvailable,
    RedrawRequested,
}

impl Video {
    pub fn new(x: f32, y: f32, width: f32, height: f32, video_subsystem: VideoSubsystem) -> Self {
        let mut mpv_gl_ctx: *mut mpv_render_context = ptr::null_mut();
        let mut framebuffer: GLuint = GLuint::default();
        let mut media_texture: GLuint = GLuint::default();
        let mut shader_program: GLuint = GLuint::default();
        let mpv_handle = unsafe {
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

            libmpv_set_option_string(mpv_handle, "background", "#00000000");
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
            // libmpv_set_option_string(mpv_handle, "pulse-latency-hacks", "no");


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

            let status = mpv_render_context_create(&mut mpv_gl_ctx, mpv_handle, render_params.as_mut_ptr());
            info!("status: {}", status);
            if status < 0 {
                eprintln!("Failed to create MPV render context");
                mpv_destroy(mpv_handle);
                panic!("Failed to create MPV render context");
            }

            let raw = CString::new(format!("{} {} {}", "loadfile", VIDEO_URL, "replace")).unwrap();

            // Send the command to MPV to load the file
            if mpv_command_string(mpv_handle, raw.as_ptr()) < 0 {
                eprintln!("Failed to load file");
                mpv_destroy(mpv_handle);
                panic!("Failed to load file");
            }

            // gl::GenFramebuffers(1, &mut framebuffer);
            // gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

            // // Create texture for the framebuffer
            // gl::GenTextures(1, &mut media_texture);
            // gl::BindTexture(gl::TEXTURE_2D, media_texture);
            //
            // // Define texture size and format
            // let (width, height) = (800, 600); // Set this to your desired dimensions
            // gl::TexImage2D(
            //     gl::TEXTURE_2D,
            //     0,
            //     gl::RGBA as GLint,
            //     width,
            //     height,
            //     0,
            //     gl::RGBA,
            //     gl::UNSIGNED_BYTE,
            //     ptr::null(),
            // );
            //
            // // Set texture parameters
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            //
            // // Attach the texture to the framebuffer
            // gl::FramebufferTexture2D(
            //     gl::FRAMEBUFFER,
            //     gl::COLOR_ATTACHMENT0,
            //     gl::TEXTURE_2D,
            //     media_texture,
            //     0,
            // );
            //
            // // Check framebuffer completeness
            // let status = gl::CheckFramebufferStatus(FRAMEBUFFER);
            // if status != gl::FRAMEBUFFER_COMPLETE {
            //     panic!("Framebuffer is not complete: {}", status);
            // }


            // gl::GenTextures(1, &mut media_texture);
            // gl::BindTexture(gl::TEXTURE_2D, media_texture);
            //
            // gl::FramebufferTexture2D(FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, media_texture, 0);
            //
            // gl::BindTexture(gl::TEXTURE_2D, 0);
            // gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            //
            // // build and compile our shader program
            // // ------------------------------------
            // // vertex shader
            // let vertex_shader = create_shader(gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            // let fragment_shader = create_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);
            //
            // shader_program = link_program(vertex_shader, fragment_shader);
            //
            // // Clean up shaders
            // gl::DeleteShader(vertex_shader);
            // gl::DeleteShader(fragment_shader);

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
            mpv_render_context: mpv_gl_ctx,
            framebuffer,
            media_texture,
            shader_program,
            mpv_handle,
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

    fn set_frame_size(&self, x: f32, y: f32, width: f32, height: f32) {
        let content_height = content_height();
        let content_width = content_width();
        let mpv_handle = self.video_data().mpv_handle;
        unsafe {
            let raw = CString::new(format!("{} {} {}", "set", "video-margin-ratio-right", (content_width - x - width) / content_width)).unwrap();

            // Send the command to MPV to load the file
            if mpv_command_string(mpv_handle, raw.as_ptr()) < 0 {
                eprintln!("Failed to load file");
                mpv_destroy(mpv_handle);
                panic!("Failed to load file");
            }

            let raw = CString::new(format!("{} {} {}", "set", "video-margin-ratio-bottom", (content_height - y - height) / content_height)).unwrap();

            // Send the command to MPV to load the file
            if mpv_command_string(mpv_handle, raw.as_ptr()) < 0 {
                eprintln!("Failed to load file");
                mpv_destroy(mpv_handle);
                panic!("Failed to load file");
            }

            let raw = CString::new(format!("{} {} {}", "set", "video-margin-ratio-top", y / content_height)).unwrap();

            // Send the command to MPV to load the file
            if mpv_command_string(mpv_handle, raw.as_ptr()) < 0 {
                eprintln!("Failed to load file");
                mpv_destroy(mpv_handle);
                panic!("Failed to load file");
            }

            let raw = CString::new(format!("{} {} {}", "set", "video-margin-ratio-left", x / content_width)).unwrap();

            // Send the command to MPV to load the file
            if mpv_command_string(mpv_handle, raw.as_ptr()) < 0 {
                eprintln!("Failed to load file");
                mpv_destroy(mpv_handle);
                panic!("Failed to load file");
            }
        }
    }

    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        // todo!()
        // info!("video draw {} {}", width, height);
        let content_height = content_height();
        let content_width = content_width();
        self.set_frame_size(x, y, width, height);
        unsafe {
            // flash previous nanovg to gpu

            nvgEndFrame(ctx.vg().raw());
            nvgBeginFrame(ctx.vg().raw(), content_width, content_width, 1.5);
            gl::Viewport(0, 0, content_width as GLsizei, content_height as GLsizei);
            // nvgSave(ctx.vg().raw());
            // gl::Viewport(1280 / 4, 720 / 4, 1280 / 2, 720 / 2);

            // gl::BindFramebuffer(FRAMEBUFFER, self.video_data().framebuffer);
            // gl::Viewport(1280 / 4, 720 / 4, 1280 / 2, 720 / 2);

            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            info!("status: {}", status);
            if (status != gl::FRAMEBUFFER_COMPLETE) {
                // Handle the error (e.g., log it or throw an exception)
                panic!("Framebuffer is not complete: {}", status);
            }

            // Render video using MPV
            let mut render_params = [
                mpv_render_param {
                    type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
                    data: &mpv_opengl_fbo {
                        fbo: 0, //self.video_data().framebuffer as c_int,
                        w: 1280,
                        h: 720,
                        internal_format: gl::RGBA8 as c_int,
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

            mpv_render_context_report_swap(self.video_data().mpv_render_context);

            // // gl::UseProgram(self.video_data().shader_program);
            // // gl::BindTexture(gl::TEXTURE_2D, self.video_data().media_texture);


            // 绘制视频
            mpv_render_context_render(self.video_data().mpv_render_context, render_params.as_mut_ptr());

            gl::BindFramebuffer(FRAMEBUFFER, 0);
            gl::Viewport(0, 0, window_width() as GLsizei, window_height() as GLsizei);
            mpv_render_context_report_swap(self.video_data().mpv_render_context);


            info!("({}, {}) ({}, {}) ({}, {})", x, y, width, height, content_width, content_height);
            // nvgBeginFrame(ctx.vg().raw(), content_width, content_width, 1.5);
            // nvgBeginPath(ctx.vg().raw());
            // nvgFillColor(ctx.vg().raw(), theme("brls/background"));
            // nvgRect(ctx.vg().raw(), 0.0, 0.0, content_width, content_height);
            // nvgRect(ctx.vg().raw(), 0.0, 0.0, x, content_height);
            // nvgRect(ctx.vg().raw(), x + width, 0.0, content_width - x - width, content_height);
            // nvgRect(ctx.vg().raw(), x - 1.0, 0.0, width + 2.0 , y);
            // nvgRect(ctx.vg().raw(), x - 1.0, y + height, width + 2.0, content_height - y - height);
            // nvgFillColor(ctx.vg().raw(), nvgRGB(255, 0, 0));
            // nvgFill(ctx.vg().raw());

            // gl::Viewport(0, 0, 1280, 720);
            // nvgEndFrame(ctx.vg().raw());
            // nvgRestore(ctx.vg().raw());
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
