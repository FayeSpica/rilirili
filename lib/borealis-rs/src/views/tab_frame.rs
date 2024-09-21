use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct TabFrame;

pub trait TabFrameTrait: BoxTrait {}

impl BoxTrait for TabFrame {
    fn box_view_data(&self) -> &BoxViewData {
        todo!()
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        todo!()
    }
}

impl ViewDrawer for TabFrame {}

impl ViewLayout for TabFrame {}

impl ViewStyle for TabFrame {}

impl ViewBase for TabFrame {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl TabFrameTrait for TabFrame {}
