use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub enum AlignItems {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}

pub enum Axis {
    Row,
    Column,
}

pub enum Direction {
    Inherit,
    LeftToRight,
    RightToLeft,
}

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

impl ViewStyle for BoxView {}

impl ViewLayout for BoxView {}

impl ViewDrawer for BoxView {}
