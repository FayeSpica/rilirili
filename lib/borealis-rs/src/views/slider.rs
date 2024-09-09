use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Slider;

pub trait SliderTrait: BoxTrait {

}

impl BoxTrait for Slider {}

impl ViewDrawer for Slider {}

impl ViewLayout for Slider {}

impl ViewStyle for Slider {}

impl ViewBase for Slider {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl SliderTrait for Slider {

}