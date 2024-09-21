use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Slider;

pub trait SliderTrait: BoxTrait {}

impl BoxTrait for Slider {
    fn box_view_data(&self) -> &BoxViewData {
        todo!()
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        todo!()
    }
}

impl ViewDrawer for Slider {}

impl ViewLayout for Slider {}

impl ViewStyle for Slider {}

impl ViewBase for Slider {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl SliderTrait for Slider {}
