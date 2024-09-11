use std::collections::HashMap;
use std::{env, ptr};
use std::ffi::c_void;
use libmpv2::Mpv;
use libmpv2::render::{FBO, OpenGLInitParams, RenderContext, RenderParam, RenderParamApiType};
use nanovg_sys::{nvgBeginPath, nvgFill, nvgFillColor, nvgRect, nvgRGB};
use sdl2::{EventSubsystem, VideoSubsystem};
use sdl2::video::GLContext;
use crate::core::frame_context::FrameContext;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_box::{Axis, BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub struct Video {
    box_view_data: BoxViewData,
    mpv: Mpv,
    render_context: RenderContext,
}

fn get_proc_address(display: &sdl2::VideoSubsystem, name: &str) -> *mut c_void {
    display.gl_get_proc_address(name) as *mut c_void
}

const VIDEO_URL: &str = "./test-data/test-video.mp4";

#[derive(Debug)]
enum UserEvent {
    MpvEventAvailable,
    RedrawRequested,
}

impl Video {
    pub fn new(x: f32, y: f32, width: f32, height: f32, video: VideoSubsystem) -> Self {
        let mut mpv = Mpv::with_initializer(|init| {
            init.set_property("vo", "libmpv")?;
            Ok(())
        }).unwrap();
        let mut render_context = RenderContext::new(
            unsafe { mpv.ctx.as_mut() },
            vec![
                RenderParam::ApiType(RenderParamApiType::OpenGl),
                RenderParam::InitParams(OpenGLInitParams {
                    get_proc_address,
                    ctx: video,
                }),
            ],
        ).expect("Failed creating render context");

        mpv.event_context_mut().disable_deprecated_events().unwrap();

        let path = env::args()
            .nth(1)
            .unwrap_or_else(|| String::from(VIDEO_URL));

        info!("{}", path);

        mpv.command("loadfile", &[&path, "replace"]).unwrap();

        let s = Self {
            box_view_data: BoxViewData {
                view_data: Default::default(),
                axis: Axis::Row,
                children: vec![],
                default_focused_index: 0,
                last_focused_view: None,
                forwarded_attributes: Default::default(),
                box_view: None,
            },
            mpv,
            render_context,
        };
        s.set_width(width);
        s.set_height(height);
        s.set_position_top(x);
        s.set_position_left(y);
        s
    }
}

type DeleterFn = unsafe fn(*mut c_void);

unsafe fn free_void_data<T>(ptr: *mut c_void) {
    drop(Box::<T>::from_raw(ptr as *mut T));
}

pub trait VideoTrait: BoxTrait {

    fn video_data(&self) -> &Video;
    fn video_data_mut(&mut self) -> &mut Video;

    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        // todo!()
        info!("video draw {} {}", width, height);
        self.video_data().render_context
            .render::<VideoSubsystem>(0, width as _, height as _, true)
            .expect("Failed to draw on sdl2 window");
    }
}

impl BoxTrait for Video {}

impl ViewDrawer for Video {

}

impl ViewLayout for Video {}

impl ViewStyle for Video {}

impl ViewBase for Video {
    fn data(&self) -> &ViewData {
        &self.box_view_data.view_data
    }

    fn data_mut(&mut self) -> &mut ViewData {
        &mut self.box_view_data.view_data
    }
}

impl VideoTrait for Video {
    fn video_data(&self) -> &Video {
        self
    }

    fn video_data_mut(&mut self) -> &mut Video {
        self
    }
}
