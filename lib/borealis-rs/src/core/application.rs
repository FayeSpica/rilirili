use crate::core::activity::{Activity, ActivityDyn};
use crate::core::frame_context::FrameContext;
use crate::core::global::{
    set_content_height, set_content_width, set_window_height, set_window_scale, set_window_width,
    window_height, window_scale, window_width,
};
use crate::core::view_base::{BaseView, View};
use crate::core::view_box::BoxView;
use crate::core::view_drawer::ViewDrawer;
use nanovg::{Color, PathOptions};
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRGB, nvgRGBA, nvgRect,
};
use std::cell::RefCell;
use std::ffi::c_float;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::Arc;

const ORIGINAL_WINDOW_WIDTH: u32 = 1280;
const ORIGINAL_WINDOW_HEIGHT: u32 = 720;

pub type XMLViewCreator = Box<dyn Fn() -> BaseView>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputType {
    GAMEPAD, // Gamepad or keyboard
    TOUCH,   // Touch screen
}

pub struct Application {
    title: String,
    current_focus: Option<bool>,
    start_time: i64,
    frane_start_time: i64,
    frame_index: u64,
    global_fps: u64,
    views_to_draw: Vec<Rc<RefCell<View>>>,
    activities_stack: Vec<Rc<RefCell<Activity>>>,
    focus_stack: Vec<Rc<RefCell<View>>>,
}

impl Application {
    /**
     * Inits the borealis application.
     * Returns Ok if it succeeded, Err otherwise.
     */
    pub fn init(title: &str) -> anyhow::Result<Self> {
        let now = chrono::Local::now().timestamp_nanos();
        unsafe {
            // Init yoga
            let default_config = yoga_sys::YGConfigGetDefault();
            yoga_sys::YGConfigSetUseWebDefaults(default_config, true);
        }
        Ok(
            Application {
                title: title.into(),
                current_focus: None,
                start_time: now,
                frane_start_time: now,
                frame_index: 0,
                global_fps: 0,
                views_to_draw: vec![],
                activities_stack: vec![],
                focus_stack: vec![],
            },
        )
    }

    pub fn main_loop(self) {

    }

    pub fn internal_main_loop(&mut self) {
        self.update_fps();
        self.frane_start_time = chrono::Local::now().timestamp_nanos();

        // Render
    }

    pub fn frame(&self, ctx: &FrameContext, width: u32, height: u32, scale: f32) {
        let vg = ctx.vg().raw();
        // trace!("gl_window.window.inner_size(): {:?}", gl_window.window.inner_size());
        // trace!("gl_window.window.scale_factor(): {} {}", gl_window.window.scale_factor(), window_scale());
        unsafe {
            nvgBeginFrame(
                ctx.vg().raw(),
                width as c_float,
                height as c_float,
                scale as c_float,
            );
        }
        unsafe {
            nvgBeginPath(ctx.vg().raw());
            nvgRect(ctx.vg().raw(), 100.0, 100.0, 100.0, 100.0);
            nvgFillColor(ctx.vg().raw(), nvgRGB(255, 100, 0));
            nvgFill(ctx.vg().raw());
        }
        for view in &self.views_to_draw {
            view.borrow().frame(ctx);
        }
        unsafe {
            nvgEndFrame(ctx.vg().raw());
        }
    }

    pub fn update_fps(&mut self) {
        self.frame_index += 1;

        // update FPS every second
        if self.frane_start_time - self.start_time > 1000000000 {
            self.global_fps = self.frame_index;
            self.start_time = self.frane_start_time;
            self.frame_index = 0;
            trace!("global_fps: {}", self.global_fps);
        }
    }

    pub fn register_xml_view(&self, name: &str, creator: XMLViewCreator) {}

    pub fn push_activity(&mut self, mut activity: Activity) {
        warn!("push activity");
        activity.set_content_view(activity.create_content_view());
        activity.on_content_available();
        activity.resize_to_fit_window();
        self.views_to_draw
            .push(activity.view_data().content_view.as_ref().unwrap().clone());
        self.activities_stack.push(Rc::new(RefCell::new(activity)));
    }

    pub fn set_window_size(&self, width: u32, height: u32) {
        set_window_width(width);
        set_window_height(height);

        let scale = width as f32 / ORIGINAL_WINDOW_WIDTH as f32;

        // Rescale UI
        set_window_scale(scale);
        set_content_width(ORIGINAL_WINDOW_WIDTH as f32 * scale);
        set_content_height(ORIGINAL_WINDOW_HEIGHT as f32 * scale);

        for activity in &self.activities_stack {
            activity.borrow().on_window_size_changed();
        }
    }
}

pub fn get_input_type() -> InputType {
    InputType::TOUCH
}
