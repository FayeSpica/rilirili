use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct HScrollingFrame;

pub trait HScrollingFrameTrait: BoxTrait {}

impl BoxTrait for HScrollingFrame {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        todo!()
    }
}

impl ViewDrawer for HScrollingFrame {}

impl ViewLayout for HScrollingFrame {}

impl ViewStyle for HScrollingFrame {}

impl ViewBase for HScrollingFrame {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl HScrollingFrameTrait for HScrollingFrame {}
