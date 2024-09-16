use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Image;

pub trait ImageTrait: BoxTrait {}

impl BoxTrait for Image {}

impl ViewDrawer for Image {}

impl ViewLayout for Image {}

impl ViewStyle for Image {}

impl ViewBase for Image {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl ImageTrait for Image {}
