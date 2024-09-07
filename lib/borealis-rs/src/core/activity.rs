use std::cell::RefCell;
use crate::core::global::{content_height, content_width};
use crate::core::view_base::{View, ViewLayout};
use crate::core::view_creator::ViewCreator;
use std::path::PathBuf;
use std::rc::Rc;

pub struct ActivityViewData {
    pub xml_path: PathBuf,
    pub content_view: Option<Rc<RefCell<View>>>,
}

impl ActivityViewData {
    pub fn new() -> Self {
        Self {
            xml_path: "activity/box.xml".parse().unwrap(),
            content_view: None,
        }
    }
}

pub trait ActivityDyn: ViewCreator {
    fn view_data(&self) -> &ActivityViewData;
    fn view_data_mut(&mut self) -> &mut ActivityViewData;

    fn create_content_view(&self) -> Rc<RefCell<View>> {
        self.create_from_xml_resource(self.view_data().xml_path.clone())
    }

    fn set_content_view(&mut self, view: Rc<RefCell<View>>) {
        self.view_data_mut().content_view = Some(view);
    }

    fn resize_to_fit_window(&self) {
        match &self.view_data().content_view {
            None => {}
            Some(view) => {
                view.borrow().set_dimensions(content_width(), content_height());
            }
        }
    }

    fn on_content_available(&self) {

    }

    fn on_window_size_changed(&self) {
        self.resize_to_fit_window();
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
