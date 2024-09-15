use crate::core::activity::{Activity, ActivityDyn};
use crate::core::frame_context::FrameContext;
use crate::core::global::{set_content_height, set_content_width, set_window_height, set_window_scale, set_window_width, window_height, window_scale, window_width};
use crate::core::view_base::{BaseView, View, ViewBase};
use crate::core::view_box::BoxView;
use crate::core::view_drawer::ViewDrawer;
use nanovg::{Color, PathOptions};
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRGB, nvgRGBA, nvgRect,
};
use std::cell::RefCell;
use std::ffi::c_float;
use std::num::NonZeroU32;
use std::ptr::eq;
use std::rc::Rc;
use std::sync::Arc;
use gl::{ClearColor, FRAMEBUFFER};
use sdl2::{Sdl, VideoSubsystem};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::{GLContext, Window};
use crate::core::time::get_time_usec;
use crate::demo::activity::main_activity::MainActivity;

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
    current_focus: Option<Rc<RefCell<View>>>,
    start_time_us: i64,
    limited_frame_time_us: i64,
    frame_start_time_us: i64,
    frame_index: u64,
    global_fps: u64,
    views_to_draw: Vec<Rc<RefCell<View>>>,
    activities_stack: Vec<Rc<RefCell<Activity>>>,
    focus_stack: Vec<Rc<RefCell<View>>>,
    sdl: Sdl,
    pub video_subsystem: VideoSubsystem,
    window: Window,
    gl_context: GLContext,
    window_width: i32,
    window_height: i32,
    context: FrameContext,
    deletion_pool: Vec<Rc<RefCell<View>>>,
}

fn create_sdl2_context(title: &str, width: u32, height: u32) -> (
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::EventSubsystem,
    sdl2::VideoSubsystem,
    sdl2::video::GLContext,
) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let event_subsystem = sdl.event().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_flags().forward_compatible().set();
    let window = video
        .window(title, width, height)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let event_loop = sdl.event_pump().unwrap();

    (window, event_loop, event_subsystem, video, gl_context)
}

impl Application {
    /**
     * Inits the borealis application.
     * Returns Ok if it succeeded, Err otherwise.
     */
    pub fn init(title: &str) -> anyhow::Result<Self> {
        unsafe {
            // Init yoga
            let default_config = yoga_sys::YGConfigGetDefault();
            yoga_sys::YGConfigSetUseWebDefaults(default_config, true);
        }
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(title, 1280, 720)
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();

        // let (window, events_loop, event_subsystem, video_subsystem, gl_context) = create_sdl2_context(title, 1280, 720);
        //
        gl::load_with(|s|video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            ClearColor(0.0, 0.0, 0.0, 0.0);
        }

        let context = nanovg::ContextBuilder::new()
            .stencil_strokes()
            .antialias()
            .build()
            .unwrap();

        window.gl_swap_window();
        let now_us = get_time_usec();
        Ok(
            Application {
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
                sdl,
                video_subsystem,
                window,
                gl_context,
                window_width: 1280,
                window_height: 720,
                context: FrameContext{ context, pixel_ratio: 1.5},
                deletion_pool: vec![],
            },
        )
    }

    pub fn main_loop(mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
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
                    Event::Display { display_event,.. } => {
                        info!("Event::Display: {:?}", display_event);
                    }
                    Event::Window { win_event,.. } => {
                        info!("Event::Window: {:?}", win_event);
                        match win_event {
                            WindowEvent::None => {}
                            WindowEvent::Shown => {}
                            WindowEvent::Hidden => {}
                            WindowEvent::Exposed => {}
                            WindowEvent::Moved(_, _) => {}
                            WindowEvent::Resized(width, height) => {
                                self.window_width = width;
                                self.window_height = height;
                                self.set_window_size(width as u32, height as u32);
                                unsafe {
                                    gl::Viewport(0, 0, width, height);
                                    // ClearColor(0.75, 0.5, 0.1, 0.0);
                                    // gl::Clear(gl::COLOR_BUFFER_BIT);
                                }
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
                    Event::KeyDown { keycode,.. } => {
                        // info!("Event::KeyDown: {:?}", keycode);
                        match keycode {
                            None => {}
                            Some(code) => {
                                match code {
                                    Keycode::Equals => {
                                        self.push_activity(Activity::MainActivity(MainActivity::new(self.video_subsystem.clone())));
                                    }
                                    Keycode::Minus => {
                                        self.pop_activity();
                                    }
                                    _ => {}
                                }
                            }
                        }

                    }
                    Event::KeyUp { keycode,.. } => {
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
            self.frame(&self.context, self.window_width, self.window_height, self.context.pixel_ratio);

            // Run sync functions
            // Threading::performSyncTasks();

            // Trigger RunLoop subscribers
            // runLoopEvent.fire();

            // Free views deletion pool.
            // A view deletion might inserts other views to deletionPool
            self.deletion_pool.retain(|view|!view.borrow().ptr_locked());

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

    pub fn frame(&self, ctx: &FrameContext, width: i32, height: i32, scale: f32) {
        let vg = ctx.vg().raw();
        // trace!("gl_window.window.inner_size(): {:?}", gl_window.window.inner_size());
        // trace!("gl_window.window.scale_factor(): {} {}", gl_window.window.scale_factor(), window_scale());
        unsafe {
            gl::BindFramebuffer(FRAMEBUFFER, 0);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0); // Transparent background
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        unsafe {
            nvgBeginFrame(
                ctx.vg().raw(),
                width as c_float,
                height as c_float,
                scale as c_float,
            );
        }
        // unsafe {
        //     nvgBeginPath(ctx.vg().raw());
        //     nvgRect(ctx.vg().raw(), 100.0, 100.0, 100.0, 100.0);
        //     nvgFillColor(ctx.vg().raw(), nvgRGB(255, 100, 0));
        //     nvgFill(ctx.vg().raw());
        // }
        for view in &self.views_to_draw {
            view.borrow().frame(ctx);
        }
        unsafe {
            nvgEndFrame(ctx.vg().raw());
        }
        // will vsync depends on driver/graphics card
        self.window.gl_swap_window();
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

    pub fn register_xml_view(&self, name: &str, creator: XMLViewCreator) {}

    pub fn push_activity(&mut self, mut activity: Activity) {
        trace!("push activity");

        // Focus
        if let Some(current_focus) =  &self.current_focus {
            debug!("Pushing {} to the focus stack", current_focus.borrow().describe());
            self.focus_stack.push(current_focus.clone());
        }

        // Create the activity content view
        activity.set_content_view(activity.create_content_view());
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
        top_activity.borrow().hide(Box::new(||{}), false, 0.0);
        top_activity.borrow().on_resume();
        top_activity.borrow().show(Box::new(||{}), false, 0.0);

        // Focus
        if let Some(new_focus) = self.focus_stack.last() {

        }
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
    InputType::GAMEPAD
}
