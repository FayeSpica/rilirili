use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct HScrollingFrame;

pub trait HScrollingFrameTrait: BoxTrait {}

impl BoxTrait for HScrollingFrame {}

impl ViewDrawer for HScrollingFrame {}

impl ViewLayout for HScrollingFrame {}

impl ViewStyle for HScrollingFrame {}

impl ViewBase for HScrollingFrame {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl HScrollingFrameTrait for HScrollingFrame {}
