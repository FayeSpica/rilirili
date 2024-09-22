use std::cell::RefCell;
use std::rc::Rc;
use nanovg_sys::{nvgBeginPath, nvgFill, nvgFillColor, nvgRect};
use crate::core::attribute::register_string_xml_attribute;
use crate::core::frame_context::FrameContext;
use crate::core::style::style;
use crate::core::theme::theme;
use crate::core::view_base::{View, ViewBase, ViewData};
use crate::core::view_box::{Axis, BoxEnum, BoxTrait, BoxViewData};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::label::Label;
use crate::views::rectangle::Rectangle;
use crate::views::scrolling_frame::ScrollingFrameTrait;

pub struct SidebarSeparator {
    view_data: Rc<RefCell<ViewData>>
}

impl Default for SidebarSeparator {
    fn default() -> Self {
        let s = Self {
            view_data: Default::default(),
        };
        s.set_height(style("brls/sidebar/separator_height"));
        s
    }
}

impl SidebarSeparator {
    pub fn create() -> Rc<RefCell<View>> {
        Rc::new(RefCell::new(View::SidebarSeparator(SidebarSeparator::default())))
    }
}

pub trait SidebarSeparatorTrait: ViewTrait {}

impl ViewTrait for SidebarSeparator {}

impl ViewDrawer for SidebarSeparator {
    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        let mid_y = y + height / 2.0;
        unsafe {
            nvgBeginPath(ctx.context);
            nvgFillColor(ctx.context, theme("brls/sidebar/separator"));
            nvgRect(ctx.context, x, mid_y, width, 1.0);
            nvgFill(ctx.context);
        }
    }
}

impl ViewLayout for SidebarSeparator {}

impl ViewStyle for SidebarSeparator {}

impl ViewBase for SidebarSeparator {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl SidebarSeparatorTrait for SidebarSeparator {}

pub struct SidebarItemData {
    pub active: bool,
}

impl Default for SidebarItemData {
    fn default() -> Self {
        Self {
            active: false,
        }
    }
}

pub struct SidebarItemGroup {
    pub items: Vec<Rc<RefCell<View>>>
}

impl SidebarItemGroup {
    pub fn add(&mut self, item: Rc<RefCell<View>>) {
        self.items.push(item);
    }

    pub fn set_active(&mut self, active: Rc<RefCell<View>>) {
        for item in &self.items {
            let need_active = Rc::ptr_eq(item, &active);
            match &*item.borrow() {
                View::Box(v) => {
                    match v {
                        BoxEnum::SidebarItem(v) => v.set_active(need_active),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct SidebarItem {
    view_data: Rc<RefCell<ViewData>>,
    box_view_data: Rc<RefCell<BoxViewData>>,
    sidebar_item_data: Rc<RefCell<SidebarItemData>>,
    accent: Rc<RefCell<View>>,
    label: Rc<RefCell<View>>
}

impl Default for SidebarItem {
    fn default() -> Self {
        let mut s = Self {
            view_data: Default::default(),
            box_view_data: Default::default(),
            sidebar_item_data: Default::default(),
            accent: Rectangle::create(),
            label: Label::create(),
        };

        register_string_xml_attribute("label", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_axis(match value {
                    "row" => Axis::Row,
                    "column" => Axis::Column,
                    &_ => Axis::Row,
                }),
                _ => {}
            }
        }));

        s
    }
}

impl SidebarItem {
    pub fn create() -> Rc<RefCell<View>> {
        Rc::new(RefCell::new(View::Box(BoxEnum::SidebarItem(SidebarItem::default()))))
    }
}

pub trait SidebarItemTrait: BoxTrait {

    fn sidebar_item(&self) -> &Rc<RefCell<SidebarItemData>>;

    fn set_group(&self) {

    }

    fn set_label(&self, label: &str) {

    }

    fn set_active(&self, active: bool) {

    }
}

impl BoxTrait for SidebarItem {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        &self.box_view_data
    }
}

impl ViewDrawer for SidebarItem {}

impl ViewLayout for SidebarItem {}

impl ViewStyle for SidebarItem {}

impl ViewBase for SidebarItem {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        &self.view_data
    }
}

impl SidebarItemTrait for SidebarItem {
    fn sidebar_item(&self) -> &Rc<RefCell<SidebarItemData>> {
        &self.sidebar_item_data
    }
}

pub struct Sidebar;

pub trait SidebarTrait: ScrollingFrameTrait {}

impl ScrollingFrameTrait for Sidebar {}

impl BoxTrait for Sidebar {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        todo!()
    }
}

impl ViewDrawer for Sidebar {}

impl ViewLayout for Sidebar {}

impl ViewStyle for Sidebar {}

impl ViewBase for Sidebar {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl SidebarTrait for Sidebar {}
