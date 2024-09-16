use std::time::{SystemTime, UNIX_EPOCH};

pub type Time = u128;

/**
 * Returns the current CPU time in microseconds.
 */
pub fn get_time_usec() -> i64 {
    chrono::Local::now().timestamp_micros()
}

pub type TickingGenericCallback = Box<dyn Fn()>;

pub type TickingEndCallback = Box<dyn Fn(bool)>;
pub type TickingTickCallback = TickingGenericCallback;

/// Interface representing something that "ticks" every frame for a certain amount of frames,
/// like a timer, an animation, a background task...
/// The library manages a list of running tickings. Each ticking is reponsible for managing its own
/// lifetime by returning true or false in onUpdate.
pub trait Ticking {
    /**
     * Starts the ticking, pushing it to the list of running tickings.
     * If the ticking is finished, it will be restarted.
     * If the ticking is already running, this method will have no effect.
     */
    fn start(&self) {}

    /**
     * Stops the ticking if it was running, and executes the end callback.
     */
    fn stop(&self) {}

    /**
     * Sets a callback to be executed when the
     * ticking finishes.
     * The callback argument will be set to true if the ticking stopped
     * on its own, false if it was stopped early by the user.
     */
    fn set_end_callback(&mut self, end_callback: TickingEndCallback) {}

    /**
     * Sets a callback to be executed at every tick
     * until the ticking finishes.
     *
     * The last animation tick will execute the tick callback
     * then the end callback.
     */
    fn set_tick_callback(&mut self, tick_callback: TickingTickCallback) {}

    /**
     * Returns true if the ticking is currently running.
     */
    fn is_running(&self) -> bool {
        todo!()
    }

    /**
     * Called internally by the main loop. Takes all running tickings
     * and updates them.
     */
    fn update_tickings(&self) {
        todo!()
    }

    /**
     * Executed every frame while the ticking lives.
     * Delta is the time difference in ms between the last frame
     * and the current one.
     * Must return false if the ticking is finished and should be
     * removed from the list of active tickings.
     * The end callback will automatically be called then.
     */
    fn on_update(&self, delta: Time) {}

    /**
     * Called when the ticking becomes active.
     */
    fn on_start(&self) {}

    /**
     * Called when the ticking is stopped, either by the user
     * or because it finished.
     */
    fn on_stop(&self) {}

    fn stop_finished(&self, finished: bool) {
        todo!()
    }
}

/// Represents a "finite" ticking that runs for a known amount of time
/// and can be seek / reset / rewound.
pub trait FiniteTicking: Ticking {
    /**
     * Rewinds the ticking to go back to the beginning
     * without losing its state (as opposed to reset() that clears
     * everything in the ticking).
     * Does not start or stop it.
     */
    fn rewind(&self) {}

    /**
     * Stops and resets the ticking, clearing its state
     * in the process (as opposed to rewind() that just restarts
     * the ticking from the beginning without losing the state).
     */
    fn reset(&self) {}

    /**
     * Called when the ticking gets rewound.
     */
    fn on_rewind(&self) {}

    /**
     * Called when the ticking gets reset.
     */
    fn on_reset(&self) {}
}
