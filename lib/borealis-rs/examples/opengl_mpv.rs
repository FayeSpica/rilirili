use libmpv2::{
    render::{OpenGLInitParams, RenderContext, RenderParam, RenderParamApiType},
    Mpv,
};
use std::{env, ffi::c_void};
use std::os::raw::c_float;
use gl::types;
use gl::types::GLsizei;
use nanovg_sys::{nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRect, nvgRGB};

fn get_proc_address(display: &sdl2::VideoSubsystem, name: &str) -> *mut c_void {
    display.gl_get_proc_address(name) as *mut c_void
}

const VIDEO_URL: &str = "test-data/test-video.mp4";

#[derive(Debug)]
enum UserEvent {
    MpvEventAvailable,
    RedrawRequested,
}

fn main() {
    let (window, mut events_loop, event_subsystem, video_subsystem, _context) = create_sdl2_context();

    let gl_context = window.gl_create_context().unwrap();

    gl::load_with(|s|{
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    unsafe {
        gl::ClearColor(0.75, 0.5, 0.1, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    let context = nanovg::ContextBuilder::new()
        .stencil_strokes()
        .antialias()
        .build()
        .unwrap();

    window.gl_swap_window();

    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(VIDEO_URL));

    println!("{}", path);

    let mut mpv = Mpv::with_initializer(|init| {
        // init.set_property("config", "yes")?;
        // init.set_property("config", "yes")?;
        init.set_property("ytdl", "no")?;
        init.set_property("audio-channels", "stereo")?;
        init.set_property("idle", "yes")?;
        init.set_property("loop-file", "no")?;
        init.set_property("osd-level", "0")?;
        init.set_property("video-timing-offset", "0")?;
        init.set_property("keep-open", "yes")?;
        init.set_property("hr-seek", "yes")?;
        init.set_property("reset-on-next-file", "speed,pause")?;
        init.set_property("vo", "libmpv")?;
        // init.set_property("pulse-latency-hacks", "no")?; cause panic

        // init.set_property("hwdec", "auto-safe")?;


        init.set_property("terminal", "yes")?;
        // init.set_property("msg-level", "all=v")?;

        Ok(())
    })
        .unwrap();
    let mut render_context = RenderContext::new(
        unsafe { mpv.ctx.as_mut() },
        vec![
            RenderParam::ApiType(RenderParamApiType::OpenGl),
            RenderParam::InitParams(OpenGLInitParams {
                get_proc_address,
                ctx: video_subsystem,
            }),
        ],
    )
        .expect("Failed creating render context");

    event_subsystem
        .register_custom_event::<UserEvent>()
        .unwrap();

    mpv.event_context_mut().disable_deprecated_events().unwrap();

    let event_sender = event_subsystem.event_sender();
    render_context.set_update_callback(move || {
        event_sender
            .push_custom_event(UserEvent::RedrawRequested)
            .unwrap();
    });

    let event_sender = event_subsystem.event_sender();
    mpv.event_context_mut().set_wakeup_callback(move || {
        event_sender
            .push_custom_event(UserEvent::MpvEventAvailable)
            .unwrap();
    });
    mpv.command("loadfile", &[&path, "replace"]).unwrap();

    mpv.command("set", &["video-margin-ratio-right", "0.5"]).unwrap();
    mpv.command("set", &["video-margin-ratio-bottom", "0.5"]).unwrap();
    // mpv.command("set", &["video-margin-ratio-top", "0.5"]).unwrap();
    // mpv.command("set", &["video-margin-ratio-left", "0.5"]).unwrap();

    let framebuffer: types::GLuint = types::GLuint::default();

    'render: loop {
        for event in events_loop.poll_iter() {
            use sdl2::event::Event;

            if event.is_user_event() {
                match event.as_user_event_type::<UserEvent>().unwrap() {
                    UserEvent::RedrawRequested => {
                        let (width, height) = window.drawable_size();
                        unsafe {
                            gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                        }
                        unsafe {

                            nvgBeginFrame(
                                context.raw(),
                                width as c_float,
                                height as c_float,
                                1.0,
                            );

                            nvgBeginPath(context.raw());
                            nvgRect(context.raw(), 0.0, 0.0, 100.0, 100.0);
                            nvgFillColor(context.raw(), nvgRGB(0, 255, 0));
                            nvgFill(context.raw());
                        }


                        let (width, height) = (640, 360);
                        render_context
                            .render::<sdl2::VideoSubsystem>(0, width as _, height as _, true)
                            .expect("Failed to draw on sdl2 window");
                        //
                        // unsafe {
                        //     gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
                        //     gl::Viewport(0,0, width, height);
                        // }

                        unsafe {
                            nvgEndFrame(context.raw());
                        }

                        window.gl_swap_window();
                    }
                    UserEvent::MpvEventAvailable => loop {
                        match mpv.event_context_mut().wait_event(0.0) {
                            Some(Ok(libmpv2::events::Event::EndFile(_))) => {
                                break 'render;
                            }
                            Some(Ok(mpv_event)) => {
                                eprintln!("MPV event: {:?}", mpv_event);
                            }
                            Some(Err(err)) => {
                                eprintln!("MPV Error: {}", err);
                                break 'render;
                            }
                            None => break,
                        }
                    },
                }
            }

            match event {
                Event::Quit { .. } => {
                    break 'render;
                }
                _ => (),
            }
        }
    }
}

fn create_sdl2_context() -> (
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
        .window("OpenGL mpv", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let event_loop = sdl.event_pump().unwrap();

    (window, event_loop, event_subsystem, video, gl_context)
}
