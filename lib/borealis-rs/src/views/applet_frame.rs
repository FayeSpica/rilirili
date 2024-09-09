use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct AppletFrame;

pub trait AppletFrameTrait: BoxTrait {

}

impl BoxTrait for AppletFrame {}

impl ViewDrawer for AppletFrame {}

impl ViewLayout for AppletFrame {}

impl ViewStyle for AppletFrame {}

impl ViewBase for AppletFrame {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl AppletFrameTrait for AppletFrame{}