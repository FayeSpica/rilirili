pub struct FrameContext {
    pub context: *mut nanovg_sys::NVGcontext,
    pub pixel_ratio: f32,
    pub window_width: u32,
    pub window_height: u32,
}

impl FrameContext {
}
