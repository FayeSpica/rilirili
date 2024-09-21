use crate::core::activity::Activity;
use crate::core::animation::{Animatable, Animating};
use crate::core::application::ViewCreatorRegistry;
use crate::core::audio::Sound;
use crate::core::frame_context::FrameContext;
use crate::core::geometry::Point;
use crate::core::style::style;
use crate::core::theme;
use crate::core::time::{FiniteTicking, Ticking};
use crate::core::tweening::EasingFunction;
use crate::core::view_box::{BoxEnum, BoxTrait, BoxView};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::image::Image;
use crate::views::label::Label;
use crate::views::progress_spinner::ProgressSpinner;
use crate::views::rectangle::Rectangle;
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRect, NVGcolor,
};
use roxmltree::Node;
use std::any::{type_name, Any};
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::ffi::c_float;
use std::rc::Rc;
use yoga_sys::{YGNodeFree, YGNodeNew, YGNodeRef, YGNodeSetContext, YGNodeStyleSetHeightAuto, YGNodeStyleSetWidthAuto};
use crate::core::attribute::{AutoAttributeHandler, BoolAttributeHandler, ColorAttributeHandler, FilePathAttributeHandler, FloatAttributeHandler, StringAttributeHandler};
use crate::core::theme::transparent_color;

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
    pub parent_activity: Option<Rc<RefCell<Activity>>>,
    pub view: Option<Rc<RefCell<View>>>,
    pub ptr_lock_counter: i32,
    pub custom_focus_by_id: HashMap<FocusDirection, String>,
    pub custom_focus_by_ptr: HashMap<FocusDirection, Rc<RefCell<View>>>,
    pub culled: bool,
}

impl Drop for ViewData {
    fn drop(&mut self) {
        unsafe {
            YGNodeFree(self.yg_node);
        }
        match &self.view {
            None => {
                trace!("ViewData dropped");
            }
            Some(view) => {
                trace!("ViewData dropped with view {}", view.borrow().describe())
            }
        }
    }
}

impl Default for ViewData {
    fn default() -> Self {
        let mut s = Self {
            id: crate::core::global::gen_new_view_id(),
            background: ViewBackground::None,
            background_color: transparent_color(),
            background_start_color: transparent_color(),
            background_end_color: transparent_color(),
            background_radius: vec![0.0, 0.0, 0.0, 0.0],
            corner_radius: 0.0,
            fade_in: false,
            hidden: false,
            yg_node: unsafe {YGNodeNew()},
            alpha: Animatable::new(1.0),
            detached: false,
            detached_origin: Default::default(),
            focusable: false,
            focused: false,
            focus_sound: Sound::SoundNone,
            shadow_type: ShadowType::None,
            show_shadow: true,
            border_color: transparent_color(),
            border_thickness: 0.0,
            visibility: Visibility::Visible,
            line_color: transparent_color(),
            line_top: 0.0,
            line_left: 0.0,
            line_bottom: 0.0,
            line_right: 0.0,
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
            wireframe_enabled: false,
            parent: None,
            parent_activity: None,
            view: None,
            ptr_lock_counter: 0,
            custom_focus_by_id: Default::default(),
            custom_focus_by_ptr: Default::default(),
            culled: true,
        };

        unsafe {
            YGNodeStyleSetWidthAuto(s.yg_node);
            YGNodeStyleSetHeightAuto(s.yg_node);
        }

        s
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

    pub fn draw(&self, vg: *mut nanovg_sys::NVGcontext) {
        // 默认的绘制方法，子类可以重写此方法
        unsafe {
            nvgBeginFrame(vg, 800.0, 600.0, 1.0);
            nvgBeginPath(vg);
            nvgRect(vg, self.x, self.y, self.width, self.height);
            nvgFillColor(
                vg,
                NVGcolor {
                    rgba: [0.0, 1.0, 1.0, 1.0],
                },
            );
            nvgFill(vg);
            nvgEndFrame(vg);
        }
    }
}

pub trait ViewBase {
    fn view_data(&self) -> &Rc<RefCell<ViewData>>;

    fn on_focus_gained(&mut self) {
        trace!("on_focus_gained {}", self.describe());
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.focused = true;

        view_data_mut.highlight_alpha.reset();
        view_data_mut.highlight_alpha.add_step_easing(
            1.0,
            style("brls/animations/highlight"),
            EasingFunction::QuadraticOut,
        );
        view_data_mut.highlight_alpha.start();

        if let Some(parent) = self.parent() {
            parent.borrow_mut().on_child_focus_gained(
                parent.borrow().view().unwrap().clone(),
                parent.borrow().view().unwrap().clone(),
            );
        }
    }

    fn on_focus_lost(&mut self) {
        trace!("on_focus_lost {}", self.describe());
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.focused = false;

        view_data_mut.highlight_alpha.reset();
        view_data_mut.highlight_alpha.add_step_easing(
            0.0,
            style("brls/animations/highlight"),
            EasingFunction::QuadraticOut,
        );
        view_data_mut.highlight_alpha.start();

        if let Some(parent) = self.parent() {
            parent.borrow_mut().on_child_focus_lost(
                parent.borrow().view().unwrap().clone(),
                parent.borrow().view().unwrap().clone(),
            );
        }
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
        // todo!()
    }

    /**
     * Sets the view to be focusable.
     *
     * Required to be able to use actions that need
     * focus on that view (such as an A press).
     */
    fn set_focusable(&mut self, focusable: bool) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.focusable = focusable;
    }

    fn is_focusable(&self) -> bool {
        let view_data = self.view_data().borrow();
        view_data.focusable && view_data.visibility == Visibility::Visible
    }

    /**
     * Removes view from it's parent
     */
    fn remove_from_super_view(&self, free: bool) {
        if let Some(parent) = &self.view_data().borrow_mut().parent {
            if let Some(self_ref) = self.view() {
                parent.borrow_mut().remove_view(self_ref, free);
            }
        }
    }

    /**
     * Sets the sound to play when this view gets focused.
     */
    fn set_focus_sound(&mut self, sound: Sound) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.focus_sound = sound;
    }

    fn focus_sound(&self) -> Sound {
        let view_data = self.view_data().borrow();
        view_data.focus_sound
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
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.detached = true;
    }

    fn is_detached(&self) -> bool {
        let view_data = self.view_data().borrow();
        view_data.detached
    }

    /**
     * Sets the position of the view, if detached.
     */
    fn set_detached_position(&mut self, x: f32, y: f32) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.detached_origin.x = x;
        view_data_mut.detached_origin.y = y;
    }

    /**
     * Sets the position X of the view, if detached.
     */
    fn set_detached_position_x(&mut self, x: f32) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.detached_origin.x = x;
    }

    /**
     * Sets the position Y of the view, if detached.
     */
    fn set_detached_position_y(&mut self, y: f32) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.detached_origin.y = y;
    }

    /**
     * Gets detached position of the view.
     */
    fn detached_position(&self) -> Point {
        let view_data = self.view_data().borrow();
        view_data.detached_origin
    }

    fn has_parent(&self) -> bool {
        let view_data = self.view_data().borrow();
        view_data.parent.is_some()
    }

    fn set_parent(&mut self, parent: Option<Rc<RefCell<BoxEnum>>>) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.parent = parent;
    }

    fn parent(&self) -> Option<Rc<RefCell<BoxEnum>>> {
        let view_data = self.view_data().borrow();
        view_data.parent.clone()
    }

    /// ref to self
    fn view(&self) -> Option<Rc<RefCell<View>>> {
        let view_data = self.view_data().borrow();
        view_data.view.clone()
    }

    fn set_view(&mut self, self_ref: Option<Rc<RefCell<View>>>) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.view = self_ref;
    }

    /// free_view need two steps
    ///
    /// 1. remove loop reference
    ///     View.view = Rc<RefCell<View>> -> View.view = None
    /// 2. remove all reference
    ///     other refs = None
    ///
    /// view will not be released before reference removed
    fn free_view(&mut self) {
        self.set_view(None);
    }

    fn on_parent_focus_gained(&self, view: Rc<RefCell<View>>) {}

    fn on_parent_focus_lost(&self, view: Rc<RefCell<View>>) {}

    fn describe(&self) -> String {
        String::new()
    }

    fn ptr_lock(&mut self) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.ptr_lock_counter += 1;
    }

    fn ptr_unlock(&mut self) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.ptr_lock_counter -= 1;
    }

    fn ptr_locked(&self) -> bool {
        let view_data = self.view_data().borrow();
        view_data.ptr_lock_counter > 0
    }

    fn default_focus(&self) -> Option<Rc<RefCell<View>>> {
        if self.is_focusable() {
            return self.view();
        }
        return None;
    }

    fn parent_activity(&self) -> Option<Rc<RefCell<Activity>>> {
        let view_data = self.view_data().borrow();
        if let Some(parent_activity) = &view_data.parent_activity {
            return Some(parent_activity.clone());
        }

        if let Some(parent) = &view_data.parent {
            return parent.borrow().parent_activity();
        }

        None
    }

    fn set_parent_activity(&mut self, parent_activity: Option<Rc<RefCell<Activity>>>) {
        let mut view_data_mut = self.view_data().borrow_mut();
        view_data_mut.parent_activity = parent_activity;
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

impl Drop for View {
    fn drop(&mut self) {
        trace!("View {} dropped", self.describe());
    }
}

impl View {
    fn variant_name(&self) -> &'static str {
        match self {
            View::Box(_) => "Box",
            View::Image(_) => "Image",
            View::Label(_) => "Label",
            View::ProgressSpinner(_) => "ProgressSpinner",
            View::Rectangle(_) => "Rectangle",
        }
    }
}

impl ViewBase for View {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        match self {
            View::Box(v) => v.view_data(),
            View::Label(v) => v.view_data(),
            _ => todo!(),
        }
    }

    fn describe(&self) -> String {
        let view_data = self.view_data().borrow();
        format!("View[{}({})]", self.variant_name(), &view_data.id)
    }
}

impl ViewTrait for View {}

impl ViewDrawer for View {
    fn frame(&mut self, ctx: &FrameContext) {
        match self {
            View::Box(v) => ViewDrawer::frame(v, ctx),
            View::Image(v) => ViewDrawer::frame(v, ctx),
            View::Label(v) => ViewDrawer::frame(v, ctx),
            View::ProgressSpinner(v) => ViewDrawer::frame(v, ctx),
            View::Rectangle(v) => ViewDrawer::frame(v, ctx),
        }
    }

    fn draw(&mut self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        match self {
            View::Box(v) => BoxTrait::draw(v, ctx, x, y, width, height),
            View::Image(v) => ViewDrawer::draw(v, ctx, x, y, width, height),
            View::Label(v) => ViewDrawer::draw(v, ctx, x, y, width, height),
            View::ProgressSpinner(v) => ViewDrawer::draw(v, ctx, x, y, width, height),
            View::Rectangle(v) => ViewDrawer::draw(v, ctx, x, y, width, height),
        }
    }
}

impl ViewLayout for View {
    fn handle_xml_attributes(
        &mut self,
        element: Node,
        view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
    ) {
        match self {
            View::Box(v) => BoxTrait::handle_xml_attributes(v, element, view_creator_registry),
            View::Image(_) => todo!(),
            View::Label(_) => todo!(),
            View::ProgressSpinner(_) => todo!(),
            View::Rectangle(_) => todo!(),
        }
    }
}

impl ViewStyle for View {}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
