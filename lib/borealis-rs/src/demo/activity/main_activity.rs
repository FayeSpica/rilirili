use crate::core::activity::{ActivityDyn, ActivityViewData};
use crate::core::view_creator::ViewCreator;

pub struct MainActivity {
    activity_view_data: ActivityViewData,
}

impl MainActivity {
    pub fn new() -> Self {
        Self {
            activity_view_data: ActivityViewData::new(),
        }
    }
}

impl ViewCreator for MainActivity {}

impl ActivityDyn for MainActivity {
    fn view_data(&self) -> &ActivityViewData {
        &self.activity_view_data
    }

    fn view_data_mut(&mut self) -> &mut ActivityViewData {
        &mut self.activity_view_data
    }
}
