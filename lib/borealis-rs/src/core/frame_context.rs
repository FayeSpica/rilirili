use std::sync::Mutex;
use nanovg_sys::NVGcontext;

lazy_static! {
    static ref GLOBAL_NVG_CONTEXT: Mutex<Option<FrameContext>> = Mutex::new(None);
}

pub struct FrameContext {
    pub context: *mut nanovg_sys::NVGcontext,
    pub pixel_ratio: f32,
    pub window_width: u32,
    pub window_height: u32,
}

impl FrameContext {}

unsafe impl Send for FrameContext {}
unsafe impl Sync for FrameContext {}

pub fn set_frame_context(frame_context: FrameContext) {
    let mut map = GLOBAL_NVG_CONTEXT.lock().unwrap();
    *map = Some(frame_context)
}

pub fn frame_context() -> *mut NVGcontext {
    let mut map = GLOBAL_NVG_CONTEXT.lock().unwrap();
    map.as_ref().unwrap().context
}
