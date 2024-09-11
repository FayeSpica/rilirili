use crate::core::global::{content_height, content_width};
use crate::core::view_base::{View, ViewBase};
use crate::core::view_creator::ViewCreator;
use crate::core::view_layout::ViewLayout;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use sdl2::VideoSubsystem;
use crate::core::view_box::{BoxEnum, BoxTrait, BoxView};
use crate::core::view_drawer::ViewDrawer;
use crate::views::video::Video;

pub struct ActivityViewData {
    pub xml_path: PathBuf,
    pub content_view: Option<Rc<RefCell<View>>>,
    pub video_subsystem: VideoSubsystem,
}

impl Drop for ActivityViewData {
    fn drop(&mut self) {
        trace!("ActivityViewData dropped");
        match &self.content_view {
            None => {}
            Some(view) => {
                view.borrow_mut().free_view();
            }
        }
        self.content_view = None;
    }
}

impl ActivityViewData {
    pub fn new(video_subsystem: VideoSubsystem) -> Self {
        Self {
            xml_path: "activity/box.xml".parse().unwrap(),
            content_view: None,
            video_subsystem,
        }
    }
}

pub trait ActivityDyn: ViewCreator {
    fn view_data(&self) -> &ActivityViewData;
    fn view_data_mut(&mut self) -> &mut ActivityViewData;

    fn create_content_view(&self) -> Rc<RefCell<View>> {
        // self.create_from_xml_resource(self.view_data().xml_path.clone())
        let box_view = BoxView::new(0.0, 0.0, 0.0, 0.0);
        let mut box_enum = BoxEnum::Box(box_view);
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Video(Video::new(100.0, 200.0, 640.0, 360.0, self.view_data().video_subsystem.clone()))))));

        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(BoxView::new(100.0, 100.0, 80.0, 80.0))))));
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(BoxView::new(200.0, 100.0, 80.0, 80.0))))));
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(BoxView::new(300.0, 100.0, 80.0, 80.0))))));
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(BoxView::new(400.0, 100.0, 80.0, 80.0))))));
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(BoxView::new(500.0, 100.0, 80.0, 80.0))))));

        let view = Rc::new(RefCell::new(View::Box(box_enum)));
        let view_self = view.clone();
        view.borrow_mut().set_view(Some(view_self));
        view
    }

    fn set_content_view(&mut self, view: Rc<RefCell<View>>) {
        self.view_data_mut().content_view = Some(view);
    }

    fn resize_to_fit_window(&self) {
        match &self.view_data().content_view {
            None => {}
            Some(view) => {
                view.borrow()
                    .set_dimensions(content_width(), content_height());
            }
        }
    }

    fn on_content_available(&self) {}

    fn on_window_size_changed(&self) {
        self.resize_to_fit_window();
    }

    fn will_appear(&self, reset_state: bool) {
        if let Some(content_view) = &self.view_data().content_view {
            content_view.borrow().will_appear(reset_state);
        }
    }

    fn will_disappear(&self, reset_state: bool) {
        if let Some(content_view) = &self.view_data().content_view {
            content_view.borrow().will_disappear(reset_state);
        }
    }

    fn set_in_fade_animation(&self, in_fade_animation: bool) {
        if let Some(content_view) = &self.view_data().content_view {
            content_view.borrow_mut().set_in_fade_animation(in_fade_animation);
        }
    }

    fn show(&self, cb: Box<dyn Fn()>, animate: bool, animation_duration: f32) {
        if let Some(content_view) = &self.view_data().content_view {
            content_view.borrow_mut().show_animated(cb, animate, animation_duration);
        }
    }

    fn hide(&self, cb: Box<dyn Fn()>, animate: bool, animation_duration: f32) {
        if let Some(content_view) = &self.view_data().content_view {
            content_view.borrow_mut().show_animated(cb, animate, animation_duration);
        }
    }

    fn on_pause(&self) {

    }

    fn on_resume(&self) {

    }

    fn default_focus(&self) -> Option<Rc<RefCell<View>>> {
        match &self.view_data().content_view {
            None => None,
            Some(content_view) => {
                content_view.borrow().default_focus()
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
