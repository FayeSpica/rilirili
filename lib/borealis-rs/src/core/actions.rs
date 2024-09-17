use crate::core::input::ControllerButton;
use crate::core::view_base::View;
use std::cell::RefCell;
use std::rc::Rc;

// Define a type alias for ActionListener, a function that takes a `View` reference and returns a boolean.
// Rust's function pointers or closures can be used for this.
pub type ActionListener = Box<dyn Fn(Rc<RefCell<View>>) -> bool>;

// Define ActionIdentifier as an alias for i32
pub type ActionIdentifier = i32;

// Define the Action struct
pub struct Action {
    pub button: ControllerButton,
    pub identifier: ActionIdentifier,
    pub hint_text: String,
    pub available: bool,
    pub hidden: bool,
    pub allow_repeating: bool,
    // pub sound: Sound,
    pub action_listener: ActionListener,
}
