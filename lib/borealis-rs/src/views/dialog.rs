use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Dialog;

pub trait DialogTrait: BoxTrait {}

impl BoxTrait for Dialog {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        todo!()
    }
}

impl ViewDrawer for Dialog {}

impl ViewLayout for Dialog {}

impl ViewStyle for Dialog {}

impl ViewBase for Dialog {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl DialogTrait for Dialog {}
