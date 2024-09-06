use nanovg::Context;
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRect, NVGcolor,
};
use crate::core::frame_context::FrameContext;
use crate::core::style::Style;
use crate::core::view_box::BoxView;

// common ViewData
pub struct ViewData {
    width: i32,
    height: i32,
    node: yoga_sys::YGNode,
}


pub struct BaseView {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl BaseView {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        BaseView {
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(&self, vg: &Context) {
        // 默认的绘制方法，子类可以重写此方法
        unsafe {
            nvgBeginFrame(vg.raw(), 800.0, 600.0, 1.0);
            nvgBeginPath(vg.raw());
            nvgRect(vg.raw(), self.x, self.y, self.width, self.height);
            nvgFillColor(
                vg.raw(),
                NVGcolor {
                    rgba: [0.0, 1.0, 1.0, 1.0],
                },
            );
            nvgFill(vg.raw());
            nvgEndFrame(vg.raw());
        }
    }
}

trait View {
    fn get_data(&self) -> &ViewData;
    fn frame(ctx: &FrameContext);
    fn draw(vg: &Context, x: f32, y: f32, width: f32, height: f32, style: Style, ctx: &FrameContext);
}

enum ViewEnum {
    Box(BoxView),
}
