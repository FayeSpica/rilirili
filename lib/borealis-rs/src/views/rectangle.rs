use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Rectangle;

pub trait RectangleTrait: ViewTrait {

}

impl ViewTrait for Rectangle {}

impl ViewDrawer for Rectangle {}

impl ViewLayout for Rectangle {}

impl ViewStyle for Rectangle {}

impl ViewBase for Rectangle {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl RectangleTrait for Rectangle {

}