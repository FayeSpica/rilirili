use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Rectangle;

pub trait RectangleTrait: ViewTrait {}

impl ViewTrait for Rectangle {}

impl ViewDrawer for Rectangle {}

impl ViewLayout for Rectangle {}

impl ViewStyle for Rectangle {}

impl ViewBase for Rectangle {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl RectangleTrait for Rectangle {}
