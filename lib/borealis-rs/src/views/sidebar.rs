use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::scrolling_frame::ScrollingFrameTrait;

pub struct SidebarSeparator;

pub trait SidebarSeparatorTrait: ViewTrait {}

impl ViewTrait for SidebarSeparator {}

impl ViewDrawer for SidebarSeparator {}

impl ViewLayout for SidebarSeparator {}

impl ViewStyle for SidebarSeparator {}

impl ViewBase for SidebarSeparator {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl SidebarSeparatorTrait for SidebarSeparator {}

pub struct SidebarItem;

pub trait SidebarItemTrait: BoxTrait {}

impl BoxTrait for SidebarItem {}

impl ViewDrawer for SidebarItem {}

impl ViewLayout for SidebarItem {}

impl ViewStyle for SidebarItem {}

impl ViewBase for SidebarItem {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl SidebarItemTrait for SidebarItem {}

pub struct Sidebar;

pub trait SidebarTrait: ScrollingFrameTrait {}

impl ScrollingFrameTrait for Sidebar {}

impl BoxTrait for Sidebar {}

impl ViewDrawer for Sidebar {}

impl ViewLayout for Sidebar {}

impl ViewStyle for Sidebar {}

impl ViewBase for Sidebar {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl SidebarTrait for Sidebar {}
