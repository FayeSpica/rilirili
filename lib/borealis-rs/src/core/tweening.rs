
#[derive(Debug, Copy, Clone)]
pub enum EasingFunction {
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
}

impl EasingFunction {
    fn apply(&self, t: f32) -> f32 {
        match *self {
            EasingFunction::Linear => t,
            EasingFunction::QuadraticIn => t * t,
            EasingFunction::QuadraticOut => t * (2.0 - t),
            EasingFunction::QuadraticInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFunction::CubicIn => t * t * t,
            EasingFunction::CubicOut => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            EasingFunction::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = (2.0 * t) - 2.0;
                    0.5 * t1 * t1 * t1 + 1.0
                }
            }
        }
    }
}