use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::recycler::RecyclerFrame;
use crate::views::sidebar::Sidebar;

pub enum ScrollingFrame {
    RecyclerFrame(RecyclerFrame),
    Sidebar(Sidebar),
}

pub trait ScrollingFrameTrait: BoxTrait {}

impl BoxTrait for ScrollingFrame {}

impl ViewDrawer for ScrollingFrame {}

impl ViewLayout for ScrollingFrame {}

impl ViewStyle for ScrollingFrame {}

impl ViewBase for ScrollingFrame {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl ScrollingFrameTrait for ScrollingFrame {}
