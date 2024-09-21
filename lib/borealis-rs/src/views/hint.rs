use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Hint;

pub trait HintTrait: BoxTrait {}

impl BoxTrait for Hint {
    fn box_view_data(&self) -> &BoxViewData {
        todo!()
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        todo!()
    }
}

impl ViewDrawer for Hint {}

impl ViewLayout for Hint {}

impl ViewStyle for Hint {}

impl ViewBase for Hint {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl HintTrait for Hint {}

pub struct Hints;

pub trait HintsTrait: BoxTrait {}

impl BoxTrait for Hints {
    fn box_view_data(&self) -> &BoxViewData {
        todo!()
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        todo!()
    }
}

impl ViewDrawer for Hints {}

impl ViewLayout for Hints {}

impl ViewStyle for Hints {}

impl ViewBase for Hints {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl HintsTrait for Hints {}
