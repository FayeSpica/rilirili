use nanovg::Context;
use yoga_sys::{YGNode, YGNodeNew};
use crate::core::frame_context::FrameContext;
use crate::core::style::Style;
use crate::core::theme;
use crate::core::view_base::{ShadowType, ViewBackground, ViewBase, ViewData, ViewDraw, ViewLayout};

pub struct BoxView {
    view_data: ViewData,
}

impl BoxView {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let s = Self {
            view_data: ViewData::default(),
        };
        s.set_width(width);
        s.set_height(height);
        s.set_position_top(x);
        s.set_position_left(y);
        s
    }
}

impl ViewBase for BoxView {
    fn data(&self) -> &ViewData {
        &self.view_data
    }

    fn data_mut(&mut self) -> &mut ViewData {
        &mut self.view_data
    }
}

impl ViewLayout for BoxView {}

impl ViewDraw for BoxView {}
