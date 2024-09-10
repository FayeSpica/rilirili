use crate::core::time::{FiniteTicking, Ticking};
use crate::core::tweening::EasingFunction;

/// An animatable is a float which value can be animated from an initial value to a target value,
/// during a given amount of time. An easing function can also be specified.
///
/// Declare the animatable and then use reset(initialValue) to reset the animation.
/// Add as many steps as you like by calling addStep(targetValue, duration, easing) one or multiple times.
/// Then, start and stop the animation with start() and stop().
///
/// setEndCallback() and setTickCallback() allow you to execute code as long as the animation runs and / or once when it finishes.
/// Use .getValue() to get the current value at any time.
///
/// An animatable has overloads for float conversion, comparison (==) and assignment operator (=) to allow
/// basic usage as a simple float. Assignment operator is a shortcut to the reset() method.

pub struct Animatable {
    pub current_value: f32,
}

impl Animatable {
    pub fn new(value: f32) -> Self {
        Self {
            current_value: value
        }
    }
}

impl FiniteTicking for Animatable {}

impl Ticking for Animatable {}

impl Animating for Animatable {
    fn value(&self) -> f32 {
        self.current_value
    }

    fn value_mut(&mut self) -> &mut f32 {
        &mut self.current_value
    }
}

pub trait Animating: FiniteTicking {

    /**
     * Returns the current animatable value.
     */
    fn value(&self) -> f32;

    /**
     * Returns the current animatable value.
     */
    fn value_mut(&mut self) -> &mut f32;

    /**
     * Stops and resets the animation, going back to the given initial value.
     * All steps are removed.
     * If an animation was already ongoing for that animatable, its end callback
     * will be called.
     */
    fn reset_initial(&mut self, initial_value: f32) {
        todo!()
    }

    /**
     * Stops and resets the animation. The value will stay where it's at.
     * All steps are removed.
     * If an animation was already ongoing for that animatable, its end callback
     * will be called.
     */
    fn reset(&mut self) {}

    /**
     * Adds an animation step to the target value, lasting the specified duration in milliseconds.
     *
     * An animation can have multiple steps. Target value can be greater and lower than the previous step (it can go forwards or backwards).
     * Easing function is optional, default is EasingFunction::linear.
     *
     * Duration is int32_t due to internal limitations, so a step cannot last for longer than 2 147 483 647ms.
     * The sum of the duration of all steps cannot exceed 71582min.
     */
    fn add_step_easing(&mut self, target_value: f32, duration: f32, easing: EasingFunction) {
        *self.value_mut() = target_value
    }

    /**
     * Returns the progress of the animation between 0.0f and 1.0f.
     */
    fn progress() -> f32 {
        todo!()
    }
}

pub fn update_highlight_animation() {

}

pub fn highlight_animation(gradient_x: &mut f32, gradient_y: &mut f32, gradient_z: &mut f32) {

}