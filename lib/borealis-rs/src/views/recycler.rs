use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::scrolling_frame::ScrollingFrameTrait;

pub struct RecyclerCell;

pub trait RecyclerCellTrait: BoxTrait {}

impl BoxTrait for RecyclerCell {}

impl ViewDrawer for RecyclerCell {}

impl ViewLayout for RecyclerCell {}

impl ViewStyle for RecyclerCell {}

impl ViewBase for RecyclerCell {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl RecyclerCellTrait for RecyclerCell {}

pub struct RecyclerHeader;

pub trait RecyclerHeaderTrait: RecyclerCellTrait {}

impl RecyclerCellTrait for RecyclerHeader {}

impl BoxTrait for RecyclerHeader {}

impl ViewDrawer for RecyclerHeader {}

impl ViewLayout for RecyclerHeader {}

impl ViewStyle for RecyclerHeader {}

impl ViewBase for RecyclerHeader {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl RecyclerHeaderTrait for RecyclerHeader {}

pub trait RecyclerDataSource {}

pub struct RecyclerContentBox;

pub trait RecyclerContentBoxTrait: BoxTrait {}

impl BoxTrait for RecyclerContentBox {}

impl ViewDrawer for RecyclerContentBox {}

impl ViewLayout for RecyclerContentBox {}

impl ViewStyle for RecyclerContentBox {}

impl ViewBase for RecyclerContentBox {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl RecyclerContentBoxTrait for RecyclerContentBox {}

pub struct RecyclerFrame;

pub trait RecyclerFrameTrait: ScrollingFrameTrait {}

impl ScrollingFrameTrait for RecyclerFrame {}

impl BoxTrait for RecyclerFrame {}

impl ViewDrawer for RecyclerFrame {}

impl ViewLayout for RecyclerFrame {}

impl ViewStyle for RecyclerFrame {}

impl ViewBase for RecyclerFrame {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl RecyclerFrameTrait for RecyclerFrame {}
