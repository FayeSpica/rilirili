use std::cell::RefCell;
use std::rc::Rc;
use nanovg_sys::{nvgBeginPath, NVGcolor, nvgFill, nvgFillColor, nvgRect};
use crate::core::attribute::register_color_xml_attribute;
use crate::core::frame_context::FrameContext;
use crate::core::theme::nvg_rgb;
use crate::core::view_base::{View, ViewBase, ViewData};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct RectangleData {
    color: NVGcolor
}

impl Default for RectangleData {
    fn default() -> Self {
        Self {
            color: nvg_rgb(0, 0, 255),
        }
    }
}

pub struct Rectangle {
    view_data: Rc<RefCell<ViewData>>,
    rectangle_data: Rc<RefCell<RectangleData>>,
}

impl Default for Rectangle {
    fn default() -> Self {
        let s = Self {
            view_data: Default::default(),
            rectangle_data: Default::default(),
        };

        register_color_xml_attribute("color", Box::new(|view, value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Rectangle(v) => v.set_color(value),
                _ => {}
            }
        }));

        s
    }
}

impl Rectangle {
    pub fn create() -> Rc<RefCell<View>> {
        Rc::new(RefCell::new(View::Rectangle(Rectangle::default())))
    }
}

pub trait RectangleTrait: ViewTrait {
    fn rectangle_data(&self) -> &Rc<RefCell<RectangleData>>;

    fn set_color(&self, color: NVGcolor) {
        self.rectangle_data().borrow_mut().color = color;
    }
}

impl ViewTrait for Rectangle {}

impl ViewDrawer for Rectangle {
    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        trace!("Rectangle draw({}, {}, {}, {}), has_parent: {}", x, y, width, height, self.has_parent());
        let color = self.a(self.rectangle_data.borrow().color);

        if color.rgba[3] == 0.0 {
            return;
        }

        unsafe {
            nvgFillColor(ctx.context, color);

            nvgBeginPath(ctx.context);
            nvgRect(ctx.context, x, y, width, height);
            nvgFill(ctx.context);
        }
    }
}

impl ViewLayout for Rectangle {}

impl ViewStyle for Rectangle {}

impl ViewBase for Rectangle {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        &self.view_data
    }
}

impl RectangleTrait for Rectangle {
    fn rectangle_data(&self) -> &Rc<RefCell<RectangleData>> {
        &self.rectangle_data
    }
}
