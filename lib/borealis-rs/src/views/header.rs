use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Header;

pub trait HeaderTrait: BoxTrait {

}

impl BoxTrait for Header {}

impl ViewDrawer for Header {}

impl ViewLayout for Header {}

impl ViewStyle for Header {}

impl ViewBase for Header {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl HeaderTrait for Header {}