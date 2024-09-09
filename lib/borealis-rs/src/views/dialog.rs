use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Dialog;

pub trait DialogTrait: BoxTrait {

}

impl BoxTrait for Dialog {}

impl ViewDrawer for Dialog {}

impl ViewLayout for Dialog {}

impl ViewStyle for Dialog {}

impl ViewBase for Dialog {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl DialogTrait for Dialog {

}