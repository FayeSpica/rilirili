use glutin::config::{Config, ConfigSurfaceTypes, ConfigTemplate, ConfigTemplateBuilder};
use glutin::context::{ContextApi, ContextAttributesBuilder, NotCurrentContext};
use glutin::display::{Display, DisplayApiPreference, GlDisplay};
use glutin::prelude::GlConfig;
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

/// Interface to provide everything platform specific required to run borealis: graphics context, inputs, audio...
/// The best platform is automatically selected when the application starts, and cannot be changed by the user at the moment
pub trait Platform {
    /**
     * Called on startup, right after instanciation, to create and open a window
     * with the given title and size.
     */
    fn create_window(
        &self,
        title: &str,
        width: u32,
        height: u32,
        window_x_pos: f32,
        window_y_pos: f32,
    );

    /**
     *
     * This function also restores windows from maximization.
     *
     */
    fn restore_window();

    /**
     *
     * Set window size
     *
     */
    fn set_window_size(window_width: u32, window_height: u32);

    /**
     *
     * Set window size limits
     *
     */
    fn set_window_size_limits(
        window_min_width: u32,
        window_min_height: u32,
        window_max_width: u32,
        window_max_height: u32,
    );

    /**
     *
     * Set window position
     *
     */
    fn set_window_position(window_x_pos: i32, window_y_pos: i32);

    /**
     *
     * 1.restoreWindow
     * 2.Set window size
     * 3.Set window position
     *
     */
    fn set_window_state(
        window_width: u32,
        window_height: u32,
        window_x_pos: i32,
        window_y_pos: i32,
    );

    /**
     * Returns the human-readable name of the platform.
     */
    fn get_name() -> String;

    /**
     * Called at every iteration of the main.xml loop.
     * Must return false if the app should continue running
     * (for example, return false if the X button was pressed on the window).
     */
    fn main_loop_iteration() -> bool;

    fn run_loop<F>(run_loop_impl: F) -> bool
    where
        F: Fn() -> bool,
    {
        return run_loop_impl();
    }

    fn get_video_context();
}

pub fn create_window(
    event_loop: &EventLoop<()>,
    title: &str,
    width: u32,
    height: u32,
) -> (Option<Window>, Display, Option<NotCurrentContext>, Config) {
    trace!("platform create_window start");

    let raw_display = event_loop.raw_display_handle();
    let window = cfg!(wgl_backend).then(|| {
        // We create a window before the display to accommodate for WGL, since it
        // requires creating HDC for properly loading the WGL and it should be taken
        // from the window you'll be rendering into.
        WindowBuilder::new()
            .with_transparent(true)
            .build(&event_loop)
            .unwrap()
    });
    let raw_window_handle = window.as_ref().map(|w| w.raw_window_handle());

    // Create the GL display. This will create display automatically for the
    // underlying GL platform. See support module on how it's being done.
    let gl_display = crate::core::create_display(raw_display, raw_window_handle);
    println!("Running on: {}", gl_display.version_string());

    // Create the config we'll be used for window. We'll use the native window
    // raw-window-handle for it to get the right visual and use proper hdc. Note
    // that you can likely use it for other windows using the same config.
    let template = crate::core::config_template(raw_window_handle);
    let config = unsafe { gl_display.find_configs(template) }
        .unwrap()
        .reduce(|accum, config| {
            // Find the config with the maximum number of samples.
            //
            // In general if you're not sure what you want in template you can request or
            // don't want to require multisampling for example, you can search for a
            // specific option you want afterwards.
            //
            // XXX however on macOS you can request only one config, so you should do
            // a search with the help of `find_configs` and adjusting your template.

            // Since we try to show off transparency try to pick the config that supports it
            // on X11 over the ones without it. XXX Configs that support
            // transparency on X11 tend to not have multisapmling, so be aware
            // of that.

            #[cfg(x11_platform)]
            let transparency_check = config
                .x11_visual()
                .map(|v| v.supports_transparency())
                .unwrap_or(false)
                & !accum
                    .x11_visual()
                    .map(|v| v.supports_transparency())
                    .unwrap_or(false);

            #[cfg(not(x11_platform))]
            let transparency_check = false;

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap();

    println!("Picked a config with {} samples", config.num_samples());

    // The context creation part. It can be created before surface and that's how
    // it's expected in multithreaded + multiwindow operation mode, since you
    // can send NotCurrentContext, but not Surface.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    let not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(&config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    });

    trace!("platform create_window end");

    (window, gl_display, not_current_gl_context, config)
}

/// Create the display.
pub fn create_display(
    raw_display: RawDisplayHandle,
    raw_window_handle: Option<RawWindowHandle>,
) -> Display {
    #[cfg(egl_backend)]
    let preference = DisplayApiPreference::Egl;

    #[cfg(glx_backend)]
    let preference = DisplayApiPreference::Glx(Box::new(unix::register_xlib_error_hook));

    #[cfg(cgl_backend)]
    let preference = DisplayApiPreference::Cgl;

    #[cfg(wgl_backend)]
    let preference = DisplayApiPreference::Wgl(Some(raw_window_handle.unwrap()));

    #[cfg(all(egl_backend, wgl_backend))]
    let preference = DisplayApiPreference::WglThenEgl(Some(raw_window_handle.unwrap()));

    #[cfg(all(egl_backend, glx_backend))]
    let preference = DisplayApiPreference::GlxThenEgl(Box::new(unix::register_xlib_error_hook));

    // Create connection to underlying OpenGL client Api.
    unsafe { Display::new(raw_display, preference).unwrap() }
}

/// Create template to find OpenGL config.
pub fn config_template(raw_window_handle: Option<RawWindowHandle>) -> ConfigTemplate {
    let mut builder = ConfigTemplateBuilder::new().with_alpha_size(8);

    if let Some(raw_window_handle) = raw_window_handle {
        builder = builder
            .compatible_with_native_window(raw_window_handle)
            .with_surface_type(ConfigSurfaceTypes::WINDOW);
    }

    #[cfg(cgl_backend)]
    let builder = builder.with_transparency(true).with_multisampling(8);

    builder.build()
}
