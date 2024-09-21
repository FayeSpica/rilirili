use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct AppletFrame;

pub trait AppletFrameTrait: BoxTrait {}

impl BoxTrait for AppletFrame {
    fn box_view_data(&self) -> &BoxViewData {
        todo!()
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        todo!()
    }
}

impl ViewDrawer for AppletFrame {}

impl ViewLayout for AppletFrame {}

impl ViewStyle for AppletFrame {}

impl ViewBase for AppletFrame {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl AppletFrameTrait for AppletFrame {}
