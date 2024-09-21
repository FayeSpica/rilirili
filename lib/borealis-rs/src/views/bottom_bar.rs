use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct BottomBar;

pub trait BottomBarTrait: BoxTrait {}

impl BoxTrait for BottomBar {
    fn box_view_data(&self) -> &BoxViewData {
        todo!()
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        todo!()
    }
}

impl ViewDrawer for BottomBar {}

impl ViewLayout for BottomBar {}

impl ViewStyle for BottomBar {}

impl ViewBase for BottomBar {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl BottomBarTrait for BottomBar {}
