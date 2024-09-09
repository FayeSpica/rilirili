use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct DebugLayer;

impl ViewDrawer for DebugLayer {}

impl ViewLayout for DebugLayer {}

impl ViewStyle for DebugLayer {}

impl ViewBase for DebugLayer {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl BoxTrait for DebugLayer {}