use nanovg_sys::NVGcolor;

pub type AutoAttributeHandler = Box<dyn Fn()>;
pub type IntAttributeHandler = Box<dyn Fn(i32)>;
pub type FloatAttributeHandler = Box<dyn Fn(f32)>;
pub type StringAttributeHandler = Box<dyn Fn(&str)>;
pub type ColorAttributeHandler = Box<dyn Fn(NVGcolor)>;
pub type BoolAttributeHandler = Box<dyn Fn(bool)>;
pub type FilePathAttributeHandler = Box<dyn Fn(&str)>;