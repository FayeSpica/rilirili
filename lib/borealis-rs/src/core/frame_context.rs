use crate::core::theme::theme;
use std::ffi::CString;
use sdl2::video::Window;

pub struct FrameContext {
    pub context: *mut nanovg_sys::NVGcontext,
    pub pixel_ratio: f32,
}

impl FrameContext {
    // pub fn new() -> Self {
    //     trace!("FrameContext::new()");
    //     let gl = gl::Gl::load_with(|symbol| {
    //         let symbol = CString::new(symbol).unwrap();
    //         gl_display.get_proc_address(symbol.as_c_str()).cast()
    //     });
    //     let clear_color = theme("brls/clear");
    //     // OpenGL 设置
    //     unsafe {
    //         gl.Viewport(0, 0, 1920, 1080); // 设置视口
    //         gl.ClearColor(
    //             clear_color.rgba[0],
    //             clear_color.rgba[1],
    //             clear_color.rgba[2],
    //             clear_color.rgba[3],
    //         ); // 设置背景色
    //     }
    //     // 初始化 NanoVG
    //     let context = nanovg::ContextBuilder::new()
    //         .stencil_strokes()
    //         .antialias()
    //         .build()
    //         .expect("glfw: unable to init nanovg");
    //
    //     Self {
    //         context,
    //         pixel_ratio: 1.0,
    //     }
    // }
}
