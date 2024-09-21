use crate::core::application::ViewCreatorRegistry;
use crate::core::view_base::{View, ViewBase, ViewData};
use crate::core::view_box::{BoxEnum, BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::recycler::RecyclerFrame;
use crate::views::sidebar::Sidebar;
use roxmltree::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub enum ScrollingFrame {
    BaseScrollingFrame(BaseScrollingFrame),
    RecyclerFrame(RecyclerFrame),
    Sidebar(Sidebar),
}

pub trait ScrollingFrameTrait: BoxTrait {}

impl BoxTrait for ScrollingFrame {
    fn box_view_data(&self) -> &BoxViewData {
        match self {
            ScrollingFrame::BaseScrollingFrame(v) => BaseScrollingFrame::box_view_data(v),
            ScrollingFrame::RecyclerFrame(v) => RecyclerFrame::box_view_data(v),
            ScrollingFrame::Sidebar(v) => Sidebar::box_view_data(v),
        }
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        match self {
            ScrollingFrame::BaseScrollingFrame(v) => BaseScrollingFrame::box_view_data_mut(v),
            ScrollingFrame::RecyclerFrame(v) => RecyclerFrame::box_view_data_mut(v),
            ScrollingFrame::Sidebar(v) => Sidebar::box_view_data_mut(v),
        }
    }
}

impl ViewDrawer for ScrollingFrame {}

impl ViewLayout for ScrollingFrame {}

impl ViewStyle for ScrollingFrame {}

impl ViewBase for ScrollingFrame {

    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        match self {
            ScrollingFrame::BaseScrollingFrame(v) => v.view_data(),
            ScrollingFrame::RecyclerFrame(v) => v.view_data(),
            ScrollingFrame::Sidebar(v) => v.view_data(),
        }
    }
}

impl ScrollingFrameTrait for ScrollingFrame {}

pub struct BaseScrollingFrame {
    box_view_data: BoxViewData,
}

impl Default for BaseScrollingFrame {
    fn default() -> Self {
        Self {
            box_view_data: Default::default(),
        }
    }
}

impl BoxTrait for BaseScrollingFrame {
    fn box_view_data(&self) -> &BoxViewData {
        &self.box_view_data
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        &mut self.box_view_data
    }
}

impl ViewDrawer for BaseScrollingFrame {}

impl ViewLayout for BaseScrollingFrame {}

impl ViewStyle for BaseScrollingFrame {}

impl ViewBase for BaseScrollingFrame {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl ScrollingFrameTrait for BaseScrollingFrame {}

impl BaseScrollingFrame {
    pub fn create() -> Rc<RefCell<View>> {
        Rc::new(RefCell::new(View::Box(BoxEnum::ScrollingFrame(
            ScrollingFrame::BaseScrollingFrame(BaseScrollingFrame::default()),
        ))))
    }
}
