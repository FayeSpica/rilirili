use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::BoxTrait;
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Hint;

pub trait HintTrait: BoxTrait {}

impl BoxTrait for Hint {}

impl ViewDrawer for Hint {}

impl ViewLayout for Hint {}

impl ViewStyle for Hint {}

impl ViewBase for Hint {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl HintTrait for Hint {}

pub struct Hints;

pub trait HintsTrait: BoxTrait {}

impl BoxTrait for Hints {}

impl ViewDrawer for Hints {}

impl ViewLayout for Hints {}

impl ViewStyle for Hints {}

impl ViewBase for Hints {
    fn data(&self) -> &ViewData {
        todo!()
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
}

impl HintsTrait for Hints {}
