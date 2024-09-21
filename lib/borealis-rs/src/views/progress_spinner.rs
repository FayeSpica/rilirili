use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct ProgressSpinner;

pub trait ProgressSpinnerTrait: ViewTrait {}

impl ViewTrait for ProgressSpinner {}

impl ViewDrawer for ProgressSpinner {}

impl ViewLayout for ProgressSpinner {}

impl ViewStyle for ProgressSpinner {}

impl ViewBase for ProgressSpinner {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl ProgressSpinnerTrait for ProgressSpinner {}
