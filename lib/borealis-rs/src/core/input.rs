use crate::core::event::Event;
use crate::core::time::Time;
use crate::core::view_base::View;
use nanovg_sys::NVGcontext;
use sdl2::rect::Point;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum BrlsKeyboardModifiers {
    BrlsKbdModifierShift = 0x01,
    BrlsKbdModifierCtrl = 0x02,
    BrlsKbdModifierAlt = 0x04,
    BrlsKbdModifierMeta = 0x08,
}

/// HidKeyboardScancode
/// Uses the same key codes as GLFW
#[derive(Debug, Clone)]
pub enum BrlsKeyboardScancode {
    /* The unknown key */
    BrlsKbdKeyUnknown = -1,

    /* Printable keys */
    BrlsKbdKeySpace = 32,
    BrlsKbdKeyApostrophe = 39, /* ' */
    BrlsKbdKeyComma = 44,      /* , */
    BrlsKbdKeyMinus = 45,      /* - */
    BrlsKbdKeyPeriod = 46,     /* . */
    BrlsKbdKeySlash = 47,      /* / */
    BrlsKbdKey0 = 48,
    BrlsKbdKey1 = 49,
    BrlsKbdKey2 = 50,
    BrlsKbdKey3 = 51,
    BrlsKbdKey4 = 52,
    BrlsKbdKey5 = 53,
    BrlsKbdKey6 = 54,
    BrlsKbdKey7 = 55,
    BrlsKbdKey8 = 56,
    BrlsKbdKey9 = 57,
    BrlsKbdKeySemicolon = 59, /* ; */
    BrlsKbdKeyEqual = 61,     /* = */
    BrlsKbdKeyA = 65,
    BrlsKbdKeyB = 66,
    BrlsKbdKeyC = 67,
    BrlsKbdKeyD = 68,
    BrlsKbdKeyE = 69,
    BrlsKbdKeyF = 70,
    BrlsKbdKeyG = 71,
    BrlsKbdKeyH = 72,
    BrlsKbdKeyI = 73,
    BrlsKbdKeyJ = 74,
    BrlsKbdKeyK = 75,
    BrlsKbdKeyL = 76,
    BrlsKbdKeyM = 77,
    BrlsKbdKeyN = 78,
    BrlsKbdKeyO = 79,
    BrlsKbdKeyP = 80,
    BrlsKbdKeyQ = 81,
    BrlsKbdKeyR = 82,
    BrlsKbdKeyS = 83,
    BrlsKbdKeyT = 84,
    BrlsKbdKeyU = 85,
    BrlsKbdKeyV = 86,
    BrlsKbdKeyW = 87,
    BrlsKbdKeyX = 88,
    BrlsKbdKeyY = 89,
    BrlsKbdKeyZ = 90,
    BrlsKbdKeyLeftBracket = 91,  /* [ */
    BrlsKbdKeyBackslash = 92,    /* \ */
    BrlsKbdKeyRightBracket = 93, /* ] */
    BrlsKbdKeyGraveAccent = 96,  /* ` */
    BrlsKbdKeyWorld1 = 161,      /* non-US #1 */
    BrlsKbdKeyWorld2 = 162,      /* non-US #2 */

    /* Function keys */
    BrlsKbdKeyEscape = 256,
    BrlsKbdKeyEnter = 257,
    BrlsKbdKeyTab = 258,
    BrlsKbdKeyBackspace = 259,
    BrlsKbdKeyInsert = 260,
    BrlsKbdKeyDelete = 261,
    BrlsKbdKeyRight = 262,
    BrlsKbdKeyLeft = 263,
    BrlsKbdKeyDown = 264,
    BrlsKbdKeyUp = 265,
    BrlsKbdKeyPageUp = 266,
    BrlsKbdKeyPageDown = 267,
    BrlsKbdKeyHome = 268,
    BrlsKbdKeyEnd = 269,
    BrlsKbdKeyCapsLock = 280,
    BrlsKbdKeyScrollLock = 281,
    BrlsKbdKeyNumLock = 282,
    BrlsKbdKeyPrintScreen = 283,
    BrlsKbdKeyPause = 284,
    BrlsKbdKeyF1 = 290,
    BrlsKbdKeyF2 = 291,
    BrlsKbdKeyF3 = 292,
    BrlsKbdKeyF4 = 293,
    BrlsKbdKeyF5 = 294,
    BrlsKbdKeyF6 = 295,
    BrlsKbdKeyF7 = 296,
    BrlsKbdKeyF8 = 297,
    BrlsKbdKeyF9 = 298,
    BrlsKbdKeyF10 = 299,
    BrlsKbdKeyF11 = 300,
    BrlsKbdKeyF12 = 301,
    BrlsKbdKeyF13 = 302,
    BrlsKbdKeyF14 = 303,
    BrlsKbdKeyF15 = 304,
    BrlsKbdKeyF16 = 305,
    BrlsKbdKeyF17 = 306,
    BrlsKbdKeyF18 = 307,
    BrlsKbdKeyF19 = 308,
    BrlsKbdKeyF20 = 309,
    BrlsKbdKeyF21 = 310,
    BrlsKbdKeyF22 = 311,
    BrlsKbdKeyF23 = 312,
    BrlsKbdKeyF24 = 313,
    BrlsKbdKeyF25 = 314,
    BrlsKbdKeyKp0 = 320,
    BrlsKbdKeyKp1 = 321,
    BrlsKbdKeyKp2 = 322,
    BrlsKbdKeyKp3 = 323,
    BrlsKbdKeyKp4 = 324,
    BrlsKbdKeyKp5 = 325,
    BrlsKbdKeyKp6 = 326,
    BrlsKbdKeyKp7 = 327,
    BrlsKbdKeyKp8 = 328,
    BrlsKbdKeyKp9 = 329,
    BrlsKbdKeyKpDecimal = 330,
    BrlsKbdKeyKpDivide = 331,
    BrlsKbdKeyKpMultiply = 332,
    BrlsKbdKeyKpSubtract = 333,
    BrlsKbdKeyKpAdd = 334,
    BrlsKbdKeyKpEnter = 335,
    BrlsKbdKeyKpEqual = 336,
    BrlsKbdKeyLeftShift = 340,
    BrlsKbdKeyLeftControl = 341,
    BrlsKbdKeyLeftAlt = 342,
    BrlsKbdKeyLeftSuper = 343,
    BrlsKbdKeyRightShift = 344,
    BrlsKbdKeyRightControl = 345,
    BrlsKbdKeyRightAlt = 346,
    BrlsKbdKeyRightSuper = 347,
    BrlsKbdKeyMenu = 348,

    BrlsKbdKeyLast,
}

/// Abstract buttons enum - names correspond to a generic Xbox controller
/// LT and RT should not be buttons but for the sake of simplicity we'll assume they are.
/// Similarly, DPAD (also called HAT) is assumed to be buttons here.
pub enum ControllerButton {
    ButtonLt = 0,
    ButtonLb,

    ButtonLsb,

    ButtonUp,
    ButtonRight,
    ButtonDown,
    ButtonLeft,

    ButtonBack,
    ButtonGuide,
    ButtonStart,

    ButtonRsb,

    ButtonY,
    ButtonB,
    ButtonA,
    ButtonX,

    ButtonRb,
    ButtonRt,

    ButtonNavUp,
    ButtonNavRight,
    ButtonNavDown,
    ButtonNavLeft,

    ButtonSpace,
    ButtonF,
    ButtonBackspace,

    ButtonMax,
}

impl ControllerButton {
    pub fn len() -> usize {
        24
    }
}

/// Abstract axis enum - names correspond to a generic Xbox controller
pub enum ControllerAxis {
    LeftX,
    LeftY,

    RightX, // also called 5th axis
    RightY, // also called 4th axis

    LeftZ,  // LT
    RightZ, // RT

    AxesMax,
}

#[derive(Debug, Clone)]
pub struct KeyState {
    key: BrlsKeyboardScancode,
    mods: u8,
    pressed: bool,
}

#[derive(Debug, Clone)]
pub enum SensorEventType {
    GYRO,
    ACCEL,
}

/// Represents the state of the controller's sensor
#[derive(Debug, Clone)]
pub struct SensorEvent {
    controller_index: usize,
    r#type: SensorEventType,
    data: [f32; 3],
    timestamp: u32,
}

/// Represents the state of the controller (a gamepad or a keyboard) in the current frame
pub struct ControllerState {
    buttons: [bool; 24],               // true: pressed
    axes: [f32; 6],                    // from 0.0f to 1.0f
    repeating_button_stop: [Time; 24], // When the pressing time is greater than this value, trigger long press or repeat
}

/// Represents a touch phase in the current frame
pub enum TouchPhase {
    START,
    STAY,
    END,
    NONE,
}

/// Contains raw touch data, filled in by platform driver
pub struct RawTouchState {
    finger_id: i32,
    pressed: bool,
    position: Point,
}

/// Contains touch data automatically filled with current phase by the library
pub struct TouchState {
    finger_id: i32,
    phase: TouchPhase,
    position: Point,
    view: Rc<RefCell<View>>,
}

/// Contains raw touch data, filled in by platform driver
pub struct RawMouseState {
    position: Point,
    offset: Point,
    scroll: Point,
    left_button: bool,
    middle_button: bool,
    right_button: bool,
}

pub struct MouseState {
    position: Point,
    offset: Point,
    scroll: Point,
    left_button: TouchPhase,
    middle_button: TouchPhase,
    right_button: TouchPhase,
    view: Rc<RefCell<View>>,
}

/// Interface responsible for reporting input state to the application - button presses,
/// axis position and touch screen state
pub trait InputManager {
    fn get_controllers_connected_count(&self) -> usize;

    fn update_unified_controller_state(&mut self, state: ControllerState);

    /**
     * Called once every frame to fill the given ControllerState struct with the controller state.
     */
    fn update_controller_state(&mut self, state: ControllerState, controller: usize);

    fn getKeyboardKeyState(&mut self, state: BrlsKeyboardScancode);

    /**
     * Called once every frame to fill the given RawTouchState struct with the raw touch data.
     */
    fn updateTouchStates(&mut self, states: Vec<RawTouchState>);

    /**
     * Called once every frame to fill the given RawTouchState struct with the raw touch data.
     */
    fn updateMouseStates(&mut self, state: RawMouseState);

    /**
     * Calls to update gamepad's rumble state.
     */
    // fn sendRumble(&mut self, controller: usize, );

    /**
     * Called once every runloop cycle to perform some cleanup before new one.
     * For internal call only
     */
    fn run_loop_start(&mut self) {}

    fn drawCursor(&mut self, vg: *mut NVGcontext) {}

    fn setPointerLock(&mut self, lock: bool) {}

    fn getMouseCusorOffsetChanged() -> Event<Point>;
    fn getMouseScrollOffsetChanged() -> Event<Point>;
    fn getControllerSensorStateChanged() -> Event<SensorEvent>;
    fn getKeyboardKeyStateChanged() -> Event<KeyState>;

    /**
     * Calculate current touch phase based on it's previous state
     */
    fn computeTouchState(
        &self,
        currentTouch: RawTouchState,
        lastFrameState: TouchState,
    ) -> TouchState;

    /**
     * Calculate current touch phase based on it's previous state
     */
    fn computeMouseState(
        &self,
        currentMouse: RawMouseState,
        lastFrameState: MouseState,
    ) -> MouseState;

    fn mapControllerState(&self, button: ControllerButton) -> ControllerButton;
}
