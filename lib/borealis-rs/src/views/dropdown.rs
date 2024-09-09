use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::recycler::RecyclerDataSource;

pub struct Dropdown;

pub trait DropdownTrait: BoxTrait + RecyclerDataSource {

}

impl BoxTrait for Dropdown {}

impl ViewDrawer for Dropdown {}

impl ViewLayout for Dropdown {}

impl ViewStyle for Dropdown {}

impl ViewBase for Dropdown {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl RecyclerDataSource for Dropdown {}

impl DropdownTrait for Dropdown {}