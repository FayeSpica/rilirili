extern crate gl;
extern crate libmpv2_sys;
extern crate sdl2;

use gl::types::GLsizei;
use libmpv2_sys::*;
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRGB, nvgRect,
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::ffi::{c_float, c_void, CStr, CString};
use std::ptr;

const VIDEO_URL: &str = "test-data/test-video.mp4";

unsafe extern "C" fn get_proc_address(ctx: *mut c_void, name: *const i8) -> *mut c_void {
    let cname = CStr::from_ptr(name);
    let sdl_video_subsystem = &*(ctx as *mut sdl2::VideoSubsystem);
    let fn_name = cname.to_str().unwrap();
    sdl_video_subsystem.gl_get_proc_address(fn_name) as *mut _
}

fn main() {
    // Initialize SDL2
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // Set OpenGL context attributes for SDL2
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // Create an SDL2 window with OpenGL
    let window = video_subsystem
        .window("MPV with SDL2", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create an OpenGL context for the window
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    let context = nanovg::ContextBuilder::new()
        .stencil_strokes()
        .antialias()
        .build()
        .unwrap();

    // Load OpenGL functions
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Initialize MPV
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

        // Set MPV to use OpenGL for rendering
        let opengl_backend = CString::new("opengl").unwrap();
        if mpv_set_option_string(
            mpv_handle,
            CString::new("vo").unwrap().as_ptr(),
            opengl_backend.as_ptr(),
        ) < 0
        {
            eprintln!("Failed to set video output backend to OpenGL");
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

        let mut mpv_gl_ctx: *mut mpv_render_context = ptr::null_mut();
        if mpv_render_context_create(&mut mpv_gl_ctx, mpv_handle, render_params.as_mut_ptr()) < 0 {
            eprintln!("Failed to create MPV render context");
            mpv_destroy(mpv_handle);
            return;
        }

        // SDL2 Event loop
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

            // Poll MPV events
            let event = mpv_wait_event(mpv_handle, 0.01);
            if !event.is_null() {
                let event_id = (*event).event_id;
                match event_id {
                    mpv_event_id_MPV_EVENT_END_FILE => {
                        println!("Video finished");
                        break 'running;
                    }
                    mpv_event_id_MPV_EVENT_SHUTDOWN => {
                        println!("MPV is shutting down");
                        break 'running;
                    }
                    _ => {}
                }
            }

            // Clear the screen
            unsafe {
                gl::Viewport(0, 0, 1280 as GLsizei, 720 as GLsizei);
                gl::ClearColor(0.0, 0.2, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            nvgBeginFrame(context.raw(), 1280 as c_float, 720 as c_float, 1.0);

            nvgBeginPath(context.raw());
            nvgRect(context.raw(), 100.0, 100.0, 100.0, 100.0);
            nvgFillColor(context.raw(), nvgRGB(0, 255, 0));
            nvgFill(context.raw());
            nvgEndFrame(context.raw());

            // // Render video using MPV
            // let mut render_params = [
            //     mpv_render_param {
            //         type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
            //         data: &mpv_opengl_fbo {
            //             fbo: 0,
            //             w: 800,
            //             h: 600,
            //             internal_format: 0,
            //         } as *const _ as *mut _,
            //     },
            //     mpv_render_param {
            //         type_: mpv_render_param_type_MPV_RENDER_PARAM_FLIP_Y,
            //         data: &1 as *const _ as *mut _,
            //     },
            //     mpv_render_param {
            //         type_: 0,
            //         data: ptr::null_mut(),
            //     },
            // ];
            // mpv_render_context_render(mpv_gl_ctx, render_params.as_mut_ptr());

            // Swap buffers to display the frame
            window.gl_swap_window();
        }

        // Clean up MPV resources
        mpv_destroy(mpv_handle);
    }
}

fn libmpv_set_option_string(mpv_handle: *mut mpv_handle, name: &str, data: &str) {
    let name = CString::new(name).unwrap();
    let data = CString::new(data).unwrap();
    unsafe {
        mpv_set_option_string(mpv_handle, name.as_ptr(), data.as_ptr());
    }
}
