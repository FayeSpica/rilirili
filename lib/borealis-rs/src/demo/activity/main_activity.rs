use crate::core::activity::{ActivityDyn, ActivityViewData};
use crate::core::view_creator::ViewCreator;
use sdl2::VideoSubsystem;

pub struct MainActivity {
    activity_view_data: ActivityViewData,
    video_subsystem: VideoSubsystem,
}

impl MainActivity {
    pub fn new(video_subsystem: VideoSubsystem) -> Self {
        Self {
            activity_view_data: ActivityViewData::new(video_subsystem.clone()),
            video_subsystem,
        }
    }
}

impl Drop for MainActivity {
    fn drop(&mut self) {
        trace!("MainActivity dropped");
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
