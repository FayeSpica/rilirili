use crate::core::theme::{set_theme_selected, ThemeVariant};

/// Interface to provide everything platform specific required to run borealis: graphics context, inputs, audio...
/// The best platform is automatically selected when the application starts, and cannot be changed by the user at the moment
pub trait PlatformDyn {
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

    fn set_theme_variant(&self, theme: ThemeVariant) {
        let theme_selected = match theme {
            ThemeVariant::LIGHT => "LIGHT",
            ThemeVariant::DARK => "DARK",
        };
        set_theme_selected(theme_selected);
    }
}

pub enum Platform {
    SDL2,
}

impl PlatformDyn for Platform {
    fn create_window(
        &self,
        title: &str,
        width: u32,
        height: u32,
        window_x_pos: f32,
        window_y_pos: f32,
    ) {
        todo!()
    }

    fn restore_window() {
        todo!()
    }

    fn set_window_size(window_width: u32, window_height: u32) {
        todo!()
    }

    fn set_window_size_limits(
        window_min_width: u32,
        window_min_height: u32,
        window_max_width: u32,
        window_max_height: u32,
    ) {
        todo!()
    }

    fn set_window_position(window_x_pos: i32, window_y_pos: i32) {
        todo!()
    }

    fn set_window_state(
        window_width: u32,
        window_height: u32,
        window_x_pos: i32,
        window_y_pos: i32,
    ) {
        todo!()
    }

    fn get_name() -> String {
        todo!()
    }

    fn main_loop_iteration() -> bool {
        todo!()
    }

    fn get_video_context() {
        todo!()
    }
}
