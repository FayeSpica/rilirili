use crate::core::frame_context::{FrameContext, set_frame_context};
use crate::core::global::{
    set_borealis_scale, set_window_height, set_window_width, window_height, window_width,
    BASE_WINDOW_WIDTH,
};
use gl::{ClearColor, FRAMEBUFFER};
use nanovg_sys::{NVGcolor, NVGcontext};
use sdl2::video::{GLContext, Window};
use sdl2::{EventPump, Sdl, VideoSubsystem};

pub struct SdlContext {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    gl_context: GLContext,
    nvg_context: *mut NVGcontext,
    frame_context: FrameContext,
    size_w: i32,
    size_h: i32,
    pos_x: i32,
    pos_y: i32,
}

impl SdlContext {
    pub fn new(title: &str) -> Self {
        unsafe {
            // Init yoga
            let default_config = yoga_sys::YGConfigGetDefault();
            yoga_sys::YGConfigSetUseWebDefaults(default_config, true);
        }
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let (window_width, window_height) = (window_width(), window_height());
        debug!("create window: ({}, {})", window_width, window_height);
        let window = video_subsystem
            .window(title, window_width, window_height)
            // .window(title, window_width(), window_height())
            // .borderless()
            // .fullscreen_desktop()
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        #[cfg(not(target_os = "android"))]
        {
            // set OpenGL 3.2 Core Profile
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 2);
            gl_attr.set_context_flags().forward_compatible().set();
        }

        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            ClearColor(0.0, 0.0, 0.0, 0.0);
        }

        let nvg_context = unsafe {
            #[cfg(target_os = "windows")]
            nanovg_sys::gladLoadGL();
            let f = nanovg_sys::NVGcreateFlags::NVG_STENCIL_STROKES
                | nanovg_sys::NVGcreateFlags::NVG_ANTIALIAS;
            #[cfg(not(target_os = "android"))]
            nanovg_sys::nvgCreateGL3(f.bits());
            #[cfg(target_os = "android")]
            nanovg_sys::nvgCreateGLES2(f.bits());
        };

        set_frame_context(FrameContext {
            context: nvg_context.clone(),
            pixel_ratio: 0.0,
            window_width: 0,
            window_height: 0,
        });

        window.gl_swap_window();

        let (window_width, window_height) = window.size();
        Self {
            sdl,
            video_subsystem,
            window,
            gl_context,
            nvg_context,
            frame_context: FrameContext {
                context: nvg_context,
                pixel_ratio: 1.0,
                window_width,
                window_height,
            },
            size_w: 0,
            size_h: 0,
            pos_x: 0,
            pos_y: 0,
        }
    }

    pub fn begin_frame(&self) {
        // nop
    }

    pub fn clear(&self, color: NVGcolor) {
        unsafe {
            gl::BindFramebuffer(FRAMEBUFFER, 0);
            gl::ClearColor(color.rgba[0], color.rgba[1], color.rgba[2], color.rgba[3]); // Transparent background
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn end_frame(&self) {
        // will vsync depends on driver/graphics card
        self.window.gl_swap_window();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn video_subsystem(&self) -> &VideoSubsystem {
        &self.video_subsystem
    }

    pub fn event_pump(&self) -> EventPump {
        self.sdl.event_pump().unwrap()
    }

    pub fn frame_context(&self) -> &FrameContext {
        &self.frame_context
    }

    pub fn frame_context_mut(&mut self) -> &mut FrameContext {
        &mut self.frame_context
    }

    pub fn sdl_window_framebuffer_size_callback(&mut self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }

        self.size_w = width;
        self.size_h = height;

        let scale = width as f32 / BASE_WINDOW_WIDTH as f32;
        // sdl window size eg: 1920*1080 => scale = 1.5
        set_window_width(width as u32);
        set_window_height(height as u32);
        set_borealis_scale(scale);

        self.frame_context_mut().window_width = width as u32;
        self.frame_context_mut().window_height = height as u32;
        self.frame_context_mut().pixel_ratio = scale;
    }

    fn get_fullscreen_bound(&mut self) {
        let display_index = self.window.display_index().unwrap();
        match self.video_subsystem.display_bounds(display_index) {
            Ok(bounds) => {
                info!("Display Bounds: {:?}", bounds);
            }
            Err(err) => {
                info!("Failed to get display bounds: {}", err);
            }
        }
    }
}
