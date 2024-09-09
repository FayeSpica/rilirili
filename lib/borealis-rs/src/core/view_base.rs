use std::cell::RefCell;
use crate::core::theme;
use crate::core::view_box::{BoxEnum, BoxTrait, BoxView};
use nanovg::Context;
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRect, NVGcolor,
};
use std::cmp::PartialEq;
use std::ffi::c_float;
use std::rc::Rc;
use yoga_sys::{YGNodeFree, YGNodeNew, YGNodeRef};
use crate::core::animation::Animatable;
use crate::core::audio::Sound;
use crate::core::geometry::Point;
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::image::Image;
use crate::views::label::Label;
use crate::views::progress_spinner::ProgressSpinner;
use crate::views::rectangle::Rectangle;

// common ViewData
pub struct ViewData {
    pub id: String,
    pub background: ViewBackground,
    pub background_color: NVGcolor,
    pub background_start_color: NVGcolor,
    pub background_end_color: NVGcolor,
    pub background_radius: Vec<f32>,
    pub corner_radius: f32,
    pub fade_in: bool,
    pub hidden: bool,
    pub yg_node: YGNodeRef,
    pub alpha: Animatable,
    pub detached: bool,
    pub detached_origin: Point,
    pub focusable: bool,
    pub focused: bool,
    pub focus_sound: Sound,
    pub shadow_type: ShadowType,
    pub show_shadow: bool,
    pub border_color: NVGcolor,
    pub border_thickness: f32,
    pub visibility: Visibility,
    pub line_color: NVGcolor,
    pub line_top: f32,
    pub line_left: f32,
    pub line_bottom: f32,
    pub line_right: f32,
    pub highlight_alpha: Animatable,
    pub highlight_corner_radius: f32,
    pub highlight_padding: f32,
    pub hide_click_animation: bool,
    pub hide_highlight_background: bool,
    pub hide_highlight_border: bool,
    pub hide_highlight: bool,
    pub click_alpha: Animatable,
    pub collapse_state: Animatable,
    pub clips_to_bounds: bool,
    pub wireframe_enabled: bool,
    pub parent: Option<Rc<RefCell<BoxEnum>>>,
    pub view: Option<Rc<RefCell<View>>>,
}

impl Default for ViewData {
    fn default() -> Self {
        Self {
            id: "fake_id".to_string(),
            background: ViewBackground::VerticalLinear,
            background_color: theme::theme("brls/background"),
            background_start_color: theme::theme("brls/background"),
            background_end_color: theme::theme("brls/background"),
            background_radius: vec![0.0, 0.0, 0.0, 0.0],
            corner_radius: 0.0,
            fade_in: false,
            hidden: false,
            yg_node: unsafe { YGNodeNew() },
            alpha: Animatable::new(1.0),
            detached: false,
            detached_origin: Default::default(),
            focusable: false,
            focused: true,
            focus_sound: Sound::SoundNone,
            shadow_type: ShadowType::Generic,
            show_shadow: true,
            border_color: theme::theme("brls/header/border"),
            border_thickness: 1.0,
            visibility: Visibility::Visible,
            line_color: theme::theme("brls/slider/line_filled"),
            line_top: 4.1,
            line_left: 4.1,
            line_bottom: 4.1,
            line_right: 4.1,
            highlight_alpha: Animatable::new(0.0),
            highlight_corner_radius: 0.0,
            highlight_padding: 0.0,
            hide_click_animation: false,
            hide_highlight_background: false,
            hide_highlight_border: false,
            hide_highlight: false,
            click_alpha: Animatable::new(0.0),
            collapse_state: Animatable::new(1.0),
            clips_to_bounds: false,
            wireframe_enabled: true,
            parent: None,
            view: None,
        }
    }
}

impl Drop for ViewData {
    fn drop(&mut self) {
        unsafe {
            YGNodeFree(self.yg_node);
        }
    }
}

pub struct BaseView {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl BaseView {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        BaseView {
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(&self, vg: &Context) {
        // 默认的绘制方法，子类可以重写此方法
        unsafe {
            nvgBeginFrame(vg.raw(), 800.0, 600.0, 1.0);
            nvgBeginPath(vg.raw());
            nvgRect(vg.raw(), self.x, self.y, self.width, self.height);
            nvgFillColor(
                vg.raw(),
                NVGcolor {
                    rgba: [0.0, 1.0, 1.0, 1.0],
                },
            );
            nvgFill(vg.raw());
            nvgEndFrame(vg.raw());
        }
    }
}

pub trait ViewBase {
    fn data(&self) -> &ViewData;
    fn data_mut(&mut self) -> &mut ViewData;

    fn on_focus_gained(&mut self) {
        self.data_mut().focused = true;
        todo!()
    }

    fn on_focus_lost(&mut self) {
        self.data_mut().focused = false;
        todo!()
    }

    fn animate_hint(&self) -> bool {
        false
    }

    fn set_background(&self, background: ViewBackground) {
        todo!()
    }

    /**
     * Returns the "nearest" view with the corresponding id, or nullptr if none has
     * been found. "Nearest" means the closest in the vicinity
     * of this view. The siblings are searched as well as its children.
     *
     * Research is done by traversing the tree upwards, starting from this view.
     * The current algorithm is very inefficient.
     */
    fn get_nearest_view(&self) -> Rc<RefCell<View>> {
        todo!()
    }

    /**
     * If set to true, will force the view to be translucent.
     */
    fn set_in_fade_animation(&mut self, translucent: bool) {
        todo!()
    }

    /**
     * Sets the view to be focusable.
     *
     * Required to be able to use actions that need
     * focus on that view (such as an A press).
     */
    fn set_focusable(&mut self, focusable: bool) {
        self.data_mut().focusable = focusable;
    }

    fn is_focusable(&self) -> bool {
        self.data().focusable && self.data().visibility == Visibility::Visible
    }

    /**
     * Removes view from it's parent
     */
    fn remove_from_super_view(&self, free: bool) {
        if let Some(parent) = &self.data().parent {
            if let Some(self_ref) = self.view() {
                parent.borrow_mut().remove_view(self_ref, free);
            }
        }
    }

    /**
     * Sets the sound to play when this view gets focused.
     */
    fn set_focus_sound(&mut self, sound: Sound) {
        self.data_mut().focus_sound = sound;
    }

    fn focus_sound(&self) -> Sound {
        self.data().focus_sound
    }

    /**
     * Sets the detached flag to true.
     * This action is irreversible.
     *
     * A detached view will, as the name suggests, not be
     * attached to their parent Yoga node. That means that invalidation
     * and layout need to be taken care of manually by the parent.
     *
     * detach() must be called before adding the view to the parent.
     */
    fn detach(&mut self) {
        self.data_mut().detached = true;
    }

    fn is_detached(&self) -> bool {
        self.data().detached
    }

    /**
     * Sets the position of the view, if detached.
     */
    fn set_detached_position(&mut self, x: f32, y: f32) {
        self.data_mut().detached_origin.x = x;
        self.data_mut().detached_origin.y = y;
    }

    /**
     * Sets the position X of the view, if detached.
     */
    fn set_detached_position_x(&mut self, x: f32) {
        self.data_mut().detached_origin.x = x;
    }

    /**
     * Sets the position Y of the view, if detached.
     */
    fn set_detached_position_y(&mut self, y: f32) {
        self.data_mut().detached_origin.y = y;
    }

    /**
     * Gets detached position of the view.
     */
    fn detached_position(&self) -> &Point {
        &self.data().detached_origin
    }

    fn has_parent(&self) -> bool {
        self.data().parent.is_some()
    }

    fn set_parent(&mut self, parent: Option<Rc<RefCell<BoxEnum>>>) {
        self.data_mut().parent = parent;
    }

    fn parent(&self) -> &Option<Rc<RefCell<BoxEnum>>> {
        &self.data().parent
    }

    /// ref to self
    fn view(&self) -> Option<Rc<RefCell<View>>> {
        self.data().view.clone()
    }

    fn set_view(&mut self, self_ref: Rc<RefCell<View>>) {
        self.data_mut().view = Some(self_ref);
    }

    fn on_parent_focus_gained(&self, view: Rc<RefCell<View>>) {

    }

    fn on_parent_focus_lost(&self, view: Rc<RefCell<View>>) {

    }

    fn describe(&self) -> &String {
        todo!()
    }

    fn free_view(&self) {

    }
}

pub fn ntz(value: f32) -> f32 {
    if value.is_nan() {
        return 0.0;
    }
    value
}

pub enum View {
    Box(BoxEnum),
    Image(Image),
    Label(Label),
    ProgressSpinner(ProgressSpinner),
    Rectangle(Rectangle),
}

impl ViewBase for View {
    fn data(&self) -> &ViewData {
        match self {
            View::Box(v) => v.data(),
            _ => todo!(),
        }
    }

    fn data_mut(&mut self) -> &mut ViewData {
        match self {
            View::Box(v) => v.data_mut(),
            _ => todo!(),
        }
    }
}

impl ViewTrait for View {}

impl ViewDrawer for View {}

impl ViewLayout for View {}

impl ViewStyle for View {}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum FocusDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ViewBackground {
    None,
    SideBar,
    BackDrop,
    ShapeColor,
    VerticalLinear,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PositionType {
    Relative,
    Absolute,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TransitionAnimation {
    Fade,
    SlideLeft,
    SlideRight,
    None,
    Linear,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ShadowType {
    None,
    Generic,
    Custom,
}
