use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct EditTextDialog;

pub trait EditTextDialogTrait: BoxTrait {}

impl BoxTrait for EditTextDialog {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        todo!()
    }
}

impl ViewDrawer for EditTextDialog {}

impl ViewLayout for EditTextDialog {}

impl ViewStyle for EditTextDialog {}

impl ViewBase for EditTextDialog {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl EditTextDialogTrait for EditTextDialog {}
