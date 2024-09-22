use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{View, ViewBase, ViewData};
use crate::core::view_box::{BoxEnum, BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct TabFrameData {
    active_tab: Option<Rc<RefCell<View>>>,
}

impl Default for TabFrameData {
    fn default() -> Self {
        Self {
            active_tab: None,
        }
    }
}

pub struct TabFrame {
    tab_frame_data: Rc<RefCell<TabFrameData>>,
    box_view_data: Rc<RefCell<BoxViewData>>,
    view_data: Rc<RefCell<ViewData>>
}

impl Default for TabFrame {
    fn default() -> Self {
        Self {
            tab_frame_data: Default::default(),
            box_view_data: Default::default(),
            view_data: Default::default(),
        }
    }
}

impl TabFrame {
    pub fn create() -> Rc<RefCell<View>> {
        Rc::new(RefCell::new(View::Box(BoxEnum::TabFrame(TabFrame::default()))))
    }
}

pub type TabViewCreator = Box<dyn Fn() -> Rc<RefCell<View>>>;

pub trait TabFrameTrait: BoxTrait {
    fn tab_frame_data(&self) -> &Rc<RefCell<TabFrameData>>;

    fn add_tab(&self, label: &str, creator: TabViewCreator) {

    }

    fn add_separator(&self) {

    }
}

impl BoxTrait for TabFrame {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        &self.box_view_data
    }
}

impl ViewDrawer for TabFrame {}

impl ViewLayout for TabFrame {}

impl ViewStyle for TabFrame {}

impl ViewBase for TabFrame {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        &self.view_data
    }
}

impl TabFrameTrait for TabFrame {
    fn tab_frame_data(&self) -> &Rc<RefCell<TabFrameData>> {
        &self.tab_frame_data
    }
}
