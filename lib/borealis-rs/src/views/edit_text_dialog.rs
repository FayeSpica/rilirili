use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct EditTextDialog;

pub trait EditTextDialogTrait: BoxTrait {}

impl BoxTrait for EditTextDialog {}

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
