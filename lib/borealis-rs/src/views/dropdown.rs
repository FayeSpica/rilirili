use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::recycler::RecyclerDataSource;

pub struct Dropdown;

pub trait DropdownTrait: BoxTrait + RecyclerDataSource {}

impl BoxTrait for Dropdown {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        todo!()
    }
}

impl ViewDrawer for Dropdown {}

impl ViewLayout for Dropdown {}

impl ViewStyle for Dropdown {}

impl ViewBase for Dropdown {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl RecyclerDataSource for Dropdown {}

impl DropdownTrait for Dropdown {}
