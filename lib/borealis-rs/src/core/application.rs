use crate::core::actions::ActionIdentifier;
use crate::core::activity::{Activity, ActivityDyn};
use crate::core::frame_context::FrameContext;
use crate::core::global::{
    borealis_scale, set_borealis_scale, set_window_height, set_window_width, window_height,
    window_width, BASE_WINDOW_WIDTH,
};
use crate::core::platform::Platform;
use crate::core::sdl_context::SdlContext;
use crate::core::time::get_time_usec;
use crate::core::view_base::{BaseView, View, ViewBase};
use crate::core::view_box::BoxView;
use crate::core::view_drawer::ViewDrawer;
use crate::demo::activity::main_activity::MainActivity;
use crate::demo::tab::captioned_image::CaptionedImage;
use crate::views::scrolling_frame::{BaseScrollingFrame, ScrollingFrame};
use gl::{ClearColor, FRAMEBUFFER};
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRGB, nvgRGBA, nvgRect,
    nvgScale,
};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::{GLContext, Window};
use sdl2::{Sdl, VideoSubsystem};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_float, c_int, CString};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::ptr::eq;
use std::rc::Rc;
use std::sync::Arc;
use crate::core::attribute::AttributeSetter;
use crate::core::font::add_font_stash;
use crate::core::theme::theme;
use crate::core::view_creator::{create_from_xml_file, create_from_xml_resource};
use crate::views::label::Label;

pub type XMLViewCreator = Box<dyn Fn() -> Rc<RefCell<View>>>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputType {
    GAMEPAD, // Gamepad or keyboard
    TOUCH,   // Touch screen
}

pub struct ViewCreatorRegistry {
    creators: HashMap<String, XMLViewCreator>,
}

impl ViewCreatorRegistry {
    pub fn new() -> Self {
        Self {
            creators: HashMap::new(),
        }
    }

    pub fn xml_view_creator(&self, view_name: &str) -> Option<&XMLViewCreator> {
        trace!("xml_view_creator {:?}", self.creators.keys());
        self.creators.get(view_name)
    }

    pub fn add_xml_view_creator(&mut self, view_name: &str, xml_view_creator: XMLViewCreator) {
        self.creators.insert(view_name.into(), xml_view_creator);
    }
}

pub struct Application {
    title: String,
    current_focus: Option<Rc<RefCell<View>>>,
    start_time_us: i64,
    limited_frame_time_us: i64,
    frame_start_time_us: i64,
    frame_index: u64,
    global_fps: u64,
    views_to_draw: Vec<Rc<RefCell<View>>>,
    activities_stack: Vec<Rc<RefCell<Activity>>>,
    focus_stack: Vec<Rc<RefCell<View>>>,
    sdl_context: SdlContext,
    deletion_pool: Vec<Rc<RefCell<View>>>,
    global_quit_enabled: bool,
    global_quit_identifier: ActionIdentifier,
    platform: Platform,
    view_creator_registry: Rc<RefCell<ViewCreatorRegistry>>,
    attribute_setter: AttributeSetter,
}

impl Application {
    /**
     * Inits the borealis application.
     * Returns Ok if it succeeded, Err otherwise.
     */
    pub fn create_window(title: &str) -> Self {
        let mut sdl_context = SdlContext::new(title);
        let now_us = get_time_usec();
        let mut application = Application {
            title: title.into(),
            current_focus: None,
            start_time_us: now_us,
            limited_frame_time_us: 0,
            frame_start_time_us: now_us,
            frame_index: 0,
            global_fps: 0,
            views_to_draw: vec![],
            activities_stack: vec![],
            focus_stack: vec![],
            sdl_context,
            deletion_pool: vec![],
            global_quit_enabled: false,
            global_quit_identifier: 0,
            platform: Platform::SDL2,
            view_creator_registry: Rc::new(RefCell::new(ViewCreatorRegistry::new())),
            attribute_setter: AttributeSetter::default(),
        };

        // Load fonts and setup fallbacks
        application.load_fonts();

        // Register built-in XML views
        application.register_built_in_xml_views();

        application
    }

    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    pub fn platform_mut(&mut self) -> &mut Platform {
        &mut self.platform
    }

    pub fn main_loop(mut self) {
        let mut event_pump = self.sdl_context.event_pump();
        let mut exit = false;
        while !exit {
            self.update_fps();
            self.frame_start_time_us = get_time_usec();

            while let Some(event) = event_pump.poll_event() {
                match event {
                    Event::Quit { .. } => {
                        exit = true;
                    }
                    Event::AppTerminating { .. } => {}
                    Event::AppLowMemory { .. } => {}
                    Event::AppWillEnterBackground { .. } => {}
                    Event::AppDidEnterBackground { .. } => {}
                    Event::AppWillEnterForeground { .. } => {}
                    Event::AppDidEnterForeground { .. } => {}
                    Event::Display { display_event, .. } => {
                        info!("Event::Display: {:?}", display_event);
                    }
                    Event::Window { win_event, .. } => {
                        info!("Event::Window: {:?}", win_event);
                        match win_event {
                            WindowEvent::None => {}
                            WindowEvent::Shown => {
                                let (window_width, window_height) =
                                    self.sdl_context.window().size();
                                self.on_window_size_changed(
                                    window_width as i32,
                                    window_height as i32,
                                );
                            }
                            WindowEvent::Hidden => {}
                            WindowEvent::Exposed => {}
                            WindowEvent::Moved(_, _) => {}
                            // logical size: 3840*2160 DPI 150% => 2560*1440
                            WindowEvent::Resized(window_width, window_height) => {
                                self.on_window_size_changed(window_width, window_height);
                            }
                            WindowEvent::SizeChanged(_, _) => {}
                            WindowEvent::Minimized => {}
                            WindowEvent::Maximized => {}
                            WindowEvent::Restored => {}
                            WindowEvent::Enter => {}
                            WindowEvent::Leave => {}
                            WindowEvent::FocusGained => {}
                            WindowEvent::FocusLost => {}
                            WindowEvent::Close => {}
                            WindowEvent::TakeFocus => {}
                            WindowEvent::HitTest => {}
                        }
                    }
                    Event::KeyDown { keycode, .. } => {
                        // info!("Event::KeyDown: {:?}", keycode);
                        match keycode {
                            None => {}
                            Some(code) => match code {
                                Keycode::Equals => {
                                    self.push_activity(Activity::MainActivity(MainActivity::new(
                                        self.sdl_context.video_subsystem().clone(),
                                    )));
                                }
                                Keycode::Minus => {
                                    self.pop_activity();
                                }
                                _ => {}
                            },
                        }
                    }
                    Event::KeyUp { keycode, .. } => {
                        // info!("Event::KeyUp: {:?}", keycode);
                    }
                    Event::TextEditing { .. } => {}
                    Event::TextInput { .. } => {}
                    Event::MouseMotion { .. } => {}
                    Event::MouseButtonDown { .. } => {}
                    Event::MouseButtonUp { .. } => {}
                    Event::MouseWheel { .. } => {}
                    Event::JoyAxisMotion { .. } => {}
                    Event::JoyBallMotion { .. } => {}
                    Event::JoyHatMotion { .. } => {}
                    Event::JoyButtonDown { .. } => {}
                    Event::JoyButtonUp { .. } => {}
                    Event::JoyDeviceAdded { .. } => {}
                    Event::JoyDeviceRemoved { .. } => {}
                    Event::ControllerAxisMotion { .. } => {}
                    Event::ControllerButtonDown { .. } => {}
                    Event::ControllerButtonUp { .. } => {}
                    Event::ControllerDeviceAdded { .. } => {}
                    Event::ControllerDeviceRemoved { .. } => {}
                    Event::ControllerDeviceRemapped { .. } => {}
                    Event::FingerDown { .. } => {}
                    Event::FingerUp { .. } => {}
                    Event::FingerMotion { .. } => {}
                    Event::DollarGesture { .. } => {}
                    Event::DollarRecord { .. } => {}
                    Event::MultiGesture { .. } => {}
                    Event::ClipboardUpdate { .. } => {}
                    Event::DropFile { .. } => {}
                    Event::DropText { .. } => {}
                    Event::DropBegin { .. } => {}
                    Event::DropComplete { .. } => {}
                    Event::AudioDeviceAdded { .. } => {}
                    Event::AudioDeviceRemoved { .. } => {}
                    Event::RenderTargetsReset { .. } => {}
                    Event::RenderDeviceReset { .. } => {}
                    Event::User { .. } => {}
                    Event::Unknown { .. } => {}
                    _ => {}
                }
            }

            // Ticking::updateTickings();

            // Render
            self.frame(&self.frame_context());

            // Run sync functions
            // Threading::performSyncTasks();

            // Trigger RunLoop subscribers
            // runLoopEvent.fire();

            // Free views deletion pool.
            // A view deletion might inserts other views to deletionPool
            self.deletion_pool
                .retain(|view| !view.borrow().ptr_locked());

            if self.limited_frame_time_us > 0 {
                let delta_time_us = get_time_usec() - self.frame_start_time_us;
                let interval = self.limited_frame_time_us - delta_time_us;
                // info!("{}", interval);
                if interval > 0 {
                    std::thread::sleep(std::time::Duration::from_micros(interval as u64));
                }
            }
        }
    }

    pub fn frame(&self, ctx: &FrameContext) {
        // trace!("gl_window.window.inner_size(): {:?}", gl_window.window.inner_size());
        // trace!("gl_window.window.scale_factor(): {} {}", gl_window.window.scale_factor(), window_scale());
        let background_color = theme("brls/background");
        self.sdl_context.begin_frame();
        self.sdl_context.clear(background_color);
        unsafe {
            nvgBeginFrame(
                ctx.context,
                ctx.window_width as c_float,
                ctx.window_height as c_float,
                ctx.pixel_ratio as c_float,
            );
            let scale = borealis_scale();
            nvgScale(ctx.context, scale, scale);
        }
        for view in &self.views_to_draw {
            view.borrow_mut().frame(ctx);
        }
        unsafe {
            nvgEndFrame(ctx.context);
        }
        self.sdl_context.end_frame();
    }

    /// manually limit fps
    pub fn set_limited_fps(&mut self, fps: i64) {
        self.limited_frame_time_us = 1000000 / fps
    }

    pub fn give_focus(&mut self, view: Rc<RefCell<View>>) {
        let new_focus = view.borrow().default_focus();
        let old_equal_new = match &self.current_focus {
            None => false,
            Some(old_focus) => Rc::ptr_eq(old_focus, &view),
        };

        if !old_equal_new {
            if let Some(old_focus) = &self.current_focus {
                old_focus.borrow_mut().on_focus_lost();
            }

            self.current_focus = new_focus;
            // globalFocusChangeEvent.fire(newFocus);

            if let Some(new_focus) = &self.current_focus {
                new_focus.borrow_mut().on_focus_lost();
                debug!("Giving focus to {}", new_focus.borrow().describe());
            }

            // globalHintsUpdateEvent.fire()
        }
    }

    pub fn update_fps(&mut self) {
        self.frame_index += 1;

        // update FPS every second
        if self.frame_start_time_us - self.start_time_us > 1000000 {
            self.global_fps = self.frame_index;
            self.start_time_us = self.frame_start_time_us;
            self.frame_index = 0;
            trace!("global_fps: {}", self.global_fps);
        }
    }

    pub fn push_activity(&mut self, mut activity: Activity) {
        trace!("push activity");

        // Focus
        if let Some(current_focus) = &self.current_focus {
            debug!(
                "Pushing {} to the focus stack",
                current_focus.borrow().describe()
            );
            self.focus_stack.push(current_focus.clone());
        }

        // Create the activity content view
        activity.set_content_view(activity.create_content_view(&self.view_creator_registry));
        activity.on_content_available();
        activity.resize_to_fit_window();

        if let Some(last) = self.activities_stack.last() {
            last.borrow().on_pause();
        }

        // Layout and prepare activity
        activity.will_appear(true);
        if let Some(default_focus) = activity.default_focus() {
            self.give_focus(default_focus);
        }

        self.views_to_draw
            .push(activity.view_data().content_view.as_ref().unwrap().clone());
        self.activities_stack.push(Rc::new(RefCell::new(activity)));
    }

    pub fn pop_activity(&mut self) {
        trace!("pop activity");
        if self.activities_stack.len() <= 1 {
            warn!("no activity pop");
            return;
        }

        let last_activity = self.activities_stack.pop().unwrap();
        last_activity.borrow().will_appear(true);
        last_activity.borrow().set_in_fade_animation(true);

        // Animate the old activity immediately
        let top_activity = self.activities_stack.last().unwrap();
        top_activity.borrow().hide(Box::new(|| {}), false, 0.0);
        top_activity.borrow().on_resume();
        top_activity.borrow().show(Box::new(|| {}), false, 0.0);

        // Focus
        if let Some(new_focus) = self.focus_stack.last() {}
    }

    pub fn on_window_size_changed(&mut self, width: i32, height: i32) {
        self.sdl_context
            .sdl_window_framebuffer_size_callback(width, height);

        for activity in &self.activities_stack {
            activity.borrow().on_window_size_changed();
        }
    }

    pub fn video_subsystem(&self) -> &VideoSubsystem {
        &self.sdl_context.video_subsystem()
    }

    pub fn frame_context(&self) -> &FrameContext {
        &self.sdl_context.frame_context()
    }

    pub fn set_global_quit(&mut self, enabled: bool) {
        self.global_quit_enabled = enabled;

        for activity in &self.activities_stack {
            if enabled {
                self.global_quit_identifier = activity.borrow_mut().register_exit_action();
            } else {
                activity
                    .borrow_mut()
                    .unregister_action(self.global_quit_identifier);
            }
        }
    }

    pub fn load_fonts(&mut self) {
        self.load_font_from_file(FONT_REGULAR, &resource("inter/Inter-Switch.ttf"))
    }

    pub fn load_font_from_file(&mut self, font_name: &str, file_path: &str) {
        info!("load_font_from_file({}, {})", font_name, file_path);
        let f_name = CString::new(font_name).unwrap();
        let f_path = CString::new(file_path).unwrap();
        let handle = unsafe {
            nanovg_sys::nvgCreateFont(self.frame_context().context, f_name.as_ptr(), f_path.as_ptr())
        };

        if handle == -1 {
            panic!("nvgCreateFont failed");
        }

        add_font_stash(font_name, handle);
    }

    pub fn register_built_in_xml_views(&mut self) {
        self.register_xml_view("ScrollingFrame", Box::new(BaseScrollingFrame::create));
        self.register_xml_view("Box", Box::new(BoxView::create));
        self.register_xml_view("Rectangle", Box::new(BaseScrollingFrame::create));
        self.register_xml_view("Label", Box::new(Label::create));
    }

    pub fn register_xml_view(&mut self, name: &str, creator: XMLViewCreator) {
        self.view_creator_registry
            .borrow_mut()
            .add_xml_view_creator(name.into(), creator);
    }
}

pub const FONT_REGULAR: &str = "regular";

pub fn get_input_type() -> InputType {
    InputType::GAMEPAD
}

pub fn resource(name: &str) -> String {
    let path_buf: PathBuf = PathBuf::from(crate::core::view_creator::CUSTOM_RESOURCES_PATH);
    format!("{}", path_buf.join(name).display())
}
