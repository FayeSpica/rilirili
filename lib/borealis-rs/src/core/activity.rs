use crate::core::global::{content_height, content_width};
use crate::core::view_base::{View, ViewLayout};
use crate::core::view_creator::ViewCreator;
use std::path::PathBuf;

pub struct ActivityViewData {
    xml_path: PathBuf,
    content_view: Option<View>,
}

impl ActivityViewData {
    pub fn new() -> Self {
        Self {
            xml_path: "activity/main.xml".parse().unwrap(),
            content_view: None,
        }
    }
}

pub trait ActivityDyn: ViewCreator {
    fn view_data(&self) -> &ActivityViewData;
    fn view_data_mut(&mut self) -> &mut ActivityViewData;

    fn create_content_view(&self) -> View {
        self.create_from_xml_resource(self.view_data().xml_path.clone())
    }

    fn set_content_view(&mut self, view: View) {
        self.view_data_mut().content_view = Some(view);
    }

    fn resize_to_fit_window(&mut self) {
        match &mut self.view_data_mut().content_view {
            None => {}
            Some(view) => {
                view.set_dimensions(content_width(), content_height());
            }
        }
    }
}

pub enum Activity {
    MainActivity(crate::demo::activity::main_activity::MainActivity),
}

impl ActivityDyn for Activity {
    fn view_data(&self) -> &ActivityViewData {
        match self {
            Activity::MainActivity(a) => a.view_data(),
        }
    }

    fn view_data_mut(&mut self) -> &mut ActivityViewData {
        match self {
            Activity::MainActivity(a) => a.view_data_mut(),
        }
    }
}
