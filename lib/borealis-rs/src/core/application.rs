use std::cell::RefCell;
use crate::core::activity::{Activity, ActivityDyn};
use crate::core::frame_context::FrameContext;
use crate::core::view_base::{BaseView, View, ViewDraw};
use crate::core::view_box::BoxView;
use crate::core::GlWindow;
use glutin::prelude::{GlSurface, NotCurrentGlContextSurfaceAccessor, PossiblyCurrentGlContext};
use glutin::surface::SwapInterval;
use nanovg_sys::{nvgBeginFrame, nvgEndFrame};
use std::ffi::c_float;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::Arc;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::core::global::{set_content_height, set_content_width, set_window_height, set_window_scale, set_window_width, window_height, window_scale, window_width};

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
    pub fn init() -> anyhow::Result<(Self, EventLoop<()>)> {
        let now = chrono::Local::now().timestamp_nanos();
        unsafe {
            // Init yoga
            let default_config = yoga_sys::YGConfigGetDefault();
            yoga_sys::YGConfigSetUseWebDefaults(default_config, true);
        }
        Ok((
            Application {
                title: "".to_string(),
                current_focus: None,
                start_time: now,
                frane_start_time: now,
                frame_index: 0,
                global_fps: 0,
                views_to_draw: vec![],
                activities_stack: vec![],
                focus_stack: vec![],
            },
            EventLoop::new(),
        ))
    }

    pub fn main_loop(self, event_loop: EventLoop<()>) {
        let mut state = None;
        let mut frame_context = None;
        // Create a window with a default size and position
        let (mut window, gl_display, mut not_current_gl_context, config) =
            crate::core::platform::create_window(
                &event_loop,
                "",
                ORIGINAL_WINDOW_WIDTH,
                ORIGINAL_WINDOW_HEIGHT,
            );

        event_loop.run(move |event, event_loop_window_target, control_flow| {
            control_flow.set_wait();
            // info!("{:?}", event);
            match event {
                Event::Resumed => {
                    trace!("Event::Resumed");
                    // Take a possibly early created window, or create a new one
                    let window = window.take().unwrap_or_else(|| {
                        // On X11 opacity is controlled by the visual we pass to the window latter on,
                        // other platforms decide on that by what you draw, so there's no need to pass
                        // this information to the window.
                        #[cfg(not(cgl_backend))]
                        let window = WindowBuilder::new().with_inner_size(PhysicalSize::new(window_width(), window_height()));

                        // Request opacity for window on macOS explicitly.
                        #[cfg(cgl_backend)]
                        let window = WindowBuilder::new().with_transparent(true);

                        // We must pass the visual into the X11 window upon creation, otherwise we
                        // could have mismatch errors during context activation and swap buffers.
                        #[cfg(x11_platform)]
                        let window = if let Some(visual) = config.x11_visual() {
                            window.with_x11_visual(visual.into_raw())
                        } else {
                            window
                        };

                        window.build(event_loop_window_target).unwrap()
                    });

                    // Create a wrapper for GL window and surface.
                    let gl_window = GlWindow::from_existing(&gl_display, window, &config);

                    // Make it current.
                    let gl_context = not_current_gl_context
                        .take()
                        .unwrap()
                        .make_current(&gl_window.surface)
                        .unwrap();

                    // The context needs to be current for the Renderer to set up shaders and
                    // buffers. It also performs function loading, which needs a current context on
                    // WGL.
                    frame_context.get_or_insert_with(|| FrameContext::new(&gl_display));

                    // Try setting vsync.
                    if let Err(res) = gl_window.surface.set_swap_interval(
                        &gl_context,
                        SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
                    ) {
                        eprintln!("Error setting vsync: {:?}", res);
                    }

                    assert!(state.replace((gl_context, gl_window)).is_none());
                }
                Event::Suspended => {
                    trace!("Event::Suspended");
                    // This event is only raised on Android, where the backing NativeWindow for a GL
                    // Surface can appear and disappear at any moment.
                    println!("Android window removed");

                    // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
                    // the window back to the system.
                    let (gl_context, _) = state.take().unwrap();
                    assert!(not_current_gl_context
                        .replace(gl_context.make_not_current().unwrap())
                        .is_none());
                }

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        trace!(
                            "Event::WindowEvent::Resized ({}, {})",
                            size.width,
                            size.height
                        );
                        if size.width != 0 && size.height != 0 {
                            // Some platforms like EGL require resizing GL surface to update the size
                            // Notable platforms here are Wayland and macOS, other don't require it
                            // and the function is no-op, but it's wise to resize it for portability
                            // reasons.
                            if let Some((gl_context, gl_window)) = &state {
                                gl_window.surface.resize(
                                    gl_context,
                                    NonZeroU32::new(size.width).unwrap(),
                                    NonZeroU32::new(size.height).unwrap(),
                                );
                                // let renderer = renderer.as_ref().unwrap();
                                // renderer.resize(size.width as i32, size.height as i32);
                                self.set_window_size(size.width, size.height);
                            }
                        }
                    }
                    WindowEvent::CloseRequested => {
                        trace!("Event::WindowEvent::CloseRequested");
                        control_flow.set_exit();
                    }
                    _ => {
                        // trace!("Event::WindowEvent::_");
                    }
                },
                Event::RedrawEventsCleared => {
                    // trace!("Event::RedrawEventsCleared");
                    if let Some((gl_context, gl_window)) = &state {
                        // let renderer = renderer.as_ref().unwrap();
                        // renderer.draw(gl_window);
                        let ctx =
                            frame_context.get_or_insert_with(|| FrameContext::new(&gl_display));
                        self.frame(ctx, gl_window);
                        gl_window.window.request_redraw();
                        gl_window.surface.swap_buffers(gl_context).unwrap();
                    }
                }
                _ => (),
            }
        });
    }

    pub fn internal_main_loop(&mut self) {
        self.update_fps();
        self.frane_start_time = chrono::Local::now().timestamp_nanos();

        // Render
    }

    pub fn frame(&self, ctx: &FrameContext, gl_window: &GlWindow) {
        let vg = ctx.vg().raw();
        let width = gl_window.window.inner_size().width;
        let height = gl_window.window.inner_size().height;
        // trace!("gl_window.window.inner_size(): {:?}", gl_window.window.inner_size());
        // trace!("gl_window.window.scale_factor(): {}", gl_window.window.scale_factor());
        unsafe {
            nvgBeginFrame(
                ctx.vg().raw(),
                width as c_float,
                height as c_float,
                gl_window.window.scale_factor() as c_float,
            );
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
        activity.set_content_view(activity.create_content_view());
        activity.on_content_available();
        activity.resize_to_fit_window();
        self.views_to_draw.push(activity.view_data().content_view.as_ref().unwrap().clone());
        self.activities_stack.push(Rc::new(RefCell::new(activity)));
    }

    pub fn set_window_size(&self, width: u32, height: u32) {
        set_window_width(width);
        set_window_height(height);

        // Rescale UI
        set_window_scale(width as f32 / ORIGINAL_WINDOW_WIDTH as f32);
        set_content_width(ORIGINAL_WINDOW_WIDTH as f32 * window_scale());
        set_content_height(height as f32 * window_scale());

        for activity in &self.activities_stack {
            activity.borrow().on_window_size_changed();
        }
    }
}

pub fn get_input_type() -> InputType {
    InputType::TOUCH
}
