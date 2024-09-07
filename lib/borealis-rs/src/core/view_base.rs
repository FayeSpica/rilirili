use crate::core::theme;
use crate::core::view_box::BoxView;
use nanovg::Context;
use nanovg_sys::{
    nvgBeginFrame, nvgBeginPath, NVGcolor, nvgEndFrame, nvgFill, nvgFillColor
    , nvgRect

    ,
};
use std::cmp::PartialEq;
use std::ffi::c_float;
use yoga_sys::{
    YGNodeFree

    , YGNodeNew, YGNodeRef


    ,
};

// common ViewData
pub struct ViewData {
    pub background: ViewBackground,
    pub background_color: NVGcolor,
    pub background_start_color: NVGcolor,
    pub background_end_color: NVGcolor,
    pub background_radius: Vec<f32>,
    pub corner_radius: f32,
    pub yg_node: YGNodeRef,
    pub alpha: c_float,
    pub detached: bool,
    pub focused: bool,
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
    pub highlight_alpha: f32,
    pub hide_highlight_background: bool,
    pub hide_highlight: bool,
    pub click_alpha: f32,
    pub collapse_state: f32,
    pub clips_to_bounds: bool,
    pub wireframe_enabled: bool,
}

impl Default for ViewData {
    fn default() -> Self {
        Self {
            background: ViewBackground::VerticalLinear,
            background_color: theme::theme("DARK", "brls/background"),
            background_start_color: theme::theme("DARK", "brls/background"),
            background_end_color: theme::theme("DARK", "brls/background"),
            background_radius: vec![0.0, 0.0, 0.0, 0.0],
            corner_radius: 0.0,
            yg_node: unsafe { YGNodeNew() },
            alpha: 1.0,
            detached: false,
            focused: true,
            shadow_type: ShadowType::Generic,
            show_shadow: true,
            border_color: theme::theme("DARK", "brls/header/border"),
            border_thickness: 1.0,
            visibility: Visibility::Visible,
            line_color: theme::theme("DARK", "brls/slider/line_filled"),
            line_top: 4.1,
            line_left: 4.1,
            line_bottom: 4.1,
            line_right: 4.1,
            highlight_alpha: 0.0,
            hide_highlight_background: false,
            hide_highlight: false,
            click_alpha: 0.0,
            collapse_state: 0.0,
            clips_to_bounds: false,
            wireframe_enabled: true,
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

    fn detach(&mut self) {
        self.data_mut().detached = true;
    }

    fn is_detached(&self) -> bool {
        self.data().detached
    }

    fn on_focus_gained(&mut self) {
        self.data_mut().focused = true;
    }

    fn has_parent(&self) -> bool {
        false
    }

    fn get_parent(&self) -> &View {
        todo!()
    }
}

pub fn ntz(value: f32) -> f32 {
    if value.is_nan() {
        return 0.0;
    }
    value
}

pub enum View {
    Box(BoxView),
}

impl ViewBase for View {
    fn data(&self) -> &ViewData {
        match self {
            View::Box(v) => v.data(),
        }
    }

    fn data_mut(&mut self) -> &mut ViewData {
        todo!()
    }
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
