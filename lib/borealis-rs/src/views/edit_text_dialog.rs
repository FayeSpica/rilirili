use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct EditTextDialog;

pub trait EditTextDialogTrait: BoxTrait {}

impl BoxTrait for EditTextDialog {
    fn box_view_data(&self) -> &BoxViewData {
        todo!()
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        todo!()
    }
}

impl ViewDrawer for EditTextDialog {}

impl ViewLayout for EditTextDialog {}

impl ViewStyle for EditTextDialog {}

impl ViewBase for EditTextDialog {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl EditTextDialogTrait for EditTextDialog {}
