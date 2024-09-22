use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{View, ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::image::Image;
use crate::views::label::Label;

pub struct AppletFrameData {
    view_data: Rc<RefCell<ViewData>>,
    box_view_data: Rc<RefCell<BoxViewData>>,
    content_view: Option<Rc<RefCell<View>>>,
    label: Rc<RefCell<View>>,
    icon: Rc<RefCell<View>>,
}

impl Default for AppletFrameData {
    fn default() -> Self {
        Self {
            view_data: Default::default(),
            box_view_data: Default::default(),
            content_view: None,
            label: Label::create(),
            icon: Image::create(),
        }
    }
}

pub struct AppletFrame;

pub trait AppletFrameTrait: BoxTrait {}

impl BoxTrait for AppletFrame {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        todo!()
    }
}

impl ViewDrawer for AppletFrame {}

impl ViewLayout for AppletFrame {}

impl ViewStyle for AppletFrame {}

impl ViewBase for AppletFrame {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl AppletFrameTrait for AppletFrame {}
