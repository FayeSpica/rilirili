use crate::core::activity::{ActivityDyn, ActivityViewData};
use crate::core::application::ViewCreatorRegistry;
use crate::core::view_base::View;
use crate::core::view_creator::{create_from_xml_file, create_from_xml_string};
use sdl2::VideoSubsystem;
use std::cell::RefCell;
use std::rc::Rc;

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

impl ActivityDyn for MainActivity {
    fn view_data(&self) -> &ActivityViewData {
        &self.activity_view_data
    }

    fn view_data_mut(&mut self) -> &mut ActivityViewData {
        &mut self.activity_view_data
    }

    fn create_content_view(
        &self,
        view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
    ) -> Rc<RefCell<View>> {
        // create_from_xml_file("resources/xml/activity/main.xml".parse().unwrap())
        // create_from_xml_file("resources/xml/tabs/text_test_v0.xml".parse().unwrap(), &view_creator_registry)
        // create_from_xml_file("resources/xml/tabs/text_test_v1.xml".parse().unwrap(), &view_creator_registry)
        create_from_xml_file("resources/xml/tabs/test_applet_frame.xml".parse().unwrap(), &view_creator_registry)
        // create_from_xml_string(
        //     r#"
        //     <brls:View xml="@res/xml/tabs/layout.xml" />
        // "#
        //     .into(),
        //     &view_creator_registry,
        // )
    }
}
