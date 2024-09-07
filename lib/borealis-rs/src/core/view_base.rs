use std::cmp::PartialEq;
use std::ffi::{c_float, c_uchar};
use crate::core::frame_context::FrameContext;
use crate::core::geometry::{Point, Rect, Size};
use crate::core::style::{Style, style};
use crate::core::view_box::BoxView;
use nanovg::Context;
use nanovg_sys::{nvgBeginFrame, nvgBeginPath, nvgEndFrame, nvgFill, nvgFillColor, nvgRect, NVGcolor, nvgLinearGradient, nvgFillPaint, nvgRoundedRect, nvgRoundedRectVarying, nvgBoxGradient, nvgRGBA, nvgPathWinding, NVGsolidity, nvgStrokeColor, nvgStrokeWidth, nvgSave, nvgRestore, nvgRGB, nvgStroke, nvgMoveTo, nvgLineTo, nvgResetScissor, nvgIntersectScissor};
use yoga_sys::{YGDirection, YGEdge, YGNodeCalculateLayout, YGNodeFree, YGNodeLayoutGetHeight, YGNodeLayoutGetLeft, YGNodeLayoutGetMargin, YGNodeLayoutGetPadding, YGNodeLayoutGetTop, YGNodeLayoutGetWidth, YGNodeNew, YGNodeRef, YGNodeStyleSetAlignSelf, YGNodeStyleSetFlexGrow, YGNodeStyleSetFlexShrink, YGNodeStyleSetHeight, YGNodeStyleSetMinHeight, YGNodeStyleSetMinHeightPercent, YGNodeStyleSetMinWidth, YGNodeStyleSetMinWidthPercent, YGNodeStyleSetPosition, YGNodeStyleSetPositionPercent, YGNodeStyleSetPositionType, YGNodeStyleSetWidth};
use yoga_sys::YGAlign::{YGAlignAuto, YGAlignBaseline, YGAlignCenter, YGAlignFlexEnd, YGAlignFlexStart, YGAlignSpaceAround, YGAlignSpaceBetween, YGAlignStretch};
use yoga_sys::YGEdge::{YGEdgeBottom, YGEdgeLeft, YGEdgeRight, YGEdgeTop};
use yoga_sys::YGPositionType::{YGPositionTypeAbsolute, YGPositionTypeRelative};
use crate::core::application::{get_input_type, InputType};
use crate::core::theme;
use crate::core::theme::{AUTO, theme, transparent_color, YG_UNDEFINED};

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
    pub wireframe_enabled: bool
}

impl Default for ViewData {
    fn default() -> Self {
        Self {
            background: ViewBackground::SideBar,
            background_color: theme::nvgRGBA(0,0 ,0 ,0),
            background_start_color: transparent_color(),
            background_end_color: theme::nvgRGBA(0,0 ,0 ,200),
            background_radius: vec![0.0, 0.0, 0.0, 0.0],
            corner_radius: 0.0,
            yg_node: unsafe{ YGNodeNew() },
            alpha: 1.0,
            detached: false,
            focused: true,
            shadow_type: ShadowType::Generic,
            show_shadow: true,
            border_color: theme::nvgRGB(255,0 ,0),
            border_thickness: 1.0,
            visibility: Visibility::Visible,
            line_color: theme::nvgRGB(0,255 ,0),
            line_top: 0.1,
            line_left: 0.1,
            line_bottom: 0.1,
            line_right: 0.1,
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

pub trait ViewLayout: ViewBase {
    fn rect(&self) -> Rect {
        return Rect::new(
            Point::new(self.x(), self.y()),
            Size::new(self.width(), self.height()),
        );
    }

    fn x(&self) -> f32 {
        return unsafe { YGNodeLayoutGetLeft(self.data().yg_node) };
    }

    fn y(&self) -> f32 {
        return unsafe { YGNodeLayoutGetTop(self.data().yg_node) };
    }

    fn local_rect(&self) -> Rect {
        return Rect::new(
            Point::new(self.local_x(), self.local_y()),
            Size::new(self.width(), self.height()),
        );
    }

    fn local_x(&self) -> f32 {
        return unsafe {
            YGNodeLayoutGetLeft(self.data().yg_node)
        }
    }

    fn local_y(&self) -> f32 {
        return unsafe {
            YGNodeLayoutGetTop(self.data().yg_node)
        }
    }

    fn width(&self) -> f32 {
        return unsafe { YGNodeLayoutGetWidth(self.data().yg_node) };
    }

    fn height(&self) -> f32 {
        return unsafe { YGNodeLayoutGetHeight(self.data().yg_node) };
    }

    fn set_width(&self, width: f32) {
        unsafe {
            YGNodeStyleSetMinWidthPercent(self.data().yg_node, 0.0);
            YGNodeStyleSetWidth(self.data().yg_node, width);
            YGNodeStyleSetMinWidth(self.data().yg_node, width);
        }
        self.invalidate();
    }

    fn set_height(&self, height: f32) {
        unsafe {
            YGNodeStyleSetMinHeightPercent(self.data().yg_node, 0.0);
            YGNodeStyleSetHeight(self.data().yg_node, height);
            YGNodeStyleSetMinHeight(self.data().yg_node, height);
        }
        self.invalidate();
    }

    fn set_position_top(&self, pos: f32) {
        unsafe {
            match pos == AUTO {
                true => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeTop, YG_UNDEFINED),
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeTop, pos),
            }
        }
        self.invalidate();
    }

    fn set_position_left(&self, pos: f32) {
        unsafe {
            match pos == AUTO {
                true => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeLeft, YG_UNDEFINED),
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeLeft, pos),
            }
        }
        self.invalidate();
    }

    fn set_position_bottom(&self, pos: f32) {
        unsafe {
            match pos == AUTO {
                true => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeBottom, YG_UNDEFINED),
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeBottom, pos),
            }
        }
        self.invalidate();
    }

    fn set_position_right(&self, pos: f32) {
        unsafe {
            match pos == AUTO {
                true => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeRight, YG_UNDEFINED),
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeRight, pos),
            }
        }
        self.invalidate();
    }

    fn set_position_top_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetPositionPercent(self.data().yg_node, YGEdge::YGEdgeTop, percentage);
        }
        self.invalidate();
    }

    fn set_position_left_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetPositionPercent(self.data().yg_node, YGEdge::YGEdgeLeft, percentage);
        }
        self.invalidate();
    }

    fn set_position_bottom_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetPositionPercent(self.data().yg_node, YGEdge::YGEdgeBottom, percentage);
        }
        self.invalidate();
    }

    fn set_position_right_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetPositionPercent(self.data().yg_node, YGEdge::YGEdgeRight, percentage);
        }
        self.invalidate();
    }

    fn set_position_type(&self, _type: PositionType) {
        unsafe {
            match _type {
                PositionType::Relative => YGNodeStyleSetPositionType(self.data().yg_node, YGPositionTypeRelative),
                PositionType::Absolute => YGNodeStyleSetPositionType(self.data().yg_node, YGPositionTypeAbsolute),
            }
        }
        self.invalidate();
    }

    fn set_grow(&self, grow: f32) {
        unsafe {
            YGNodeStyleSetFlexGrow(self.data().yg_node, grow);
        }
        self.invalidate();
    }

    fn set_shrink(&self, shrink: f32) {
        unsafe {
            YGNodeStyleSetFlexShrink(self.data().yg_node, shrink);
        }
        self.invalidate();
    }

    fn set_align_self(&self, align_self: AlignSelf) {
        unsafe {
            match align_self {
                AlignSelf::Auto => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignAuto),
                AlignSelf::FlexStart => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignFlexStart),
                AlignSelf::Center => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignCenter),
                AlignSelf::FlexEnd => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignFlexEnd),
                AlignSelf::Stretch => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignStretch),
                AlignSelf::Baseline => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignBaseline),
                AlignSelf::SpaceBetween => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignSpaceBetween),
                AlignSelf::SpaceAround => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignSpaceAround),
            }
        }
        self.invalidate();
    }

    fn invalidate(&self) {
        if self.has_parent() {
            self.get_parent().invalidate();
        } else {
            unsafe {
                YGNodeCalculateLayout(self.data().yg_node, f32::NAN, f32::NAN, YGDirection::YGDirectionLTR)
            }
        }
    }
}

pub trait ViewDraw: ViewLayout {
    fn frame(&self, ctx: &FrameContext) {
        if self.data().visibility != Visibility::Visible {
            return;
        }

        unsafe {
            nvgSave(ctx.vg().raw());
        }

        let rect = self.rect();
        trace!("rect: {:?}", rect);
        let x      = rect.min_x();
        let y      = rect.min_y();
        let width  = rect.width();
        let height = rect.height();

        if self.data().alpha > 0.0 {

            // Draw background
            self.draw_background(ctx, &rect);

            // Draw shadow
            if self.data().shadow_type != ShadowType::None && (self.data().show_shadow || get_input_type() == InputType::TOUCH) {
                self.draw_shadow(ctx, &rect);
            }

            // Draw border
            if self.data().border_thickness > 0.0 {
                self.draw_border(ctx, &rect);
            }

            self.draw_line(ctx, &rect);

            // Draw highlight background
            if self.data().highlight_alpha > 0.0 && !self.data().hide_highlight_background && !self.data().hide_highlight {
                self.draw_highlight(ctx, &rect, self.alpha(), true);
            }

            // Draw click animation
            if self.data().click_alpha > 0.0 {
                self.draw_click_animation(ctx, &rect);
            }

            // Collapse clipping
            if self.data().collapse_state < 1.0 || self.data().clips_to_bounds {
                unsafe {
                    nvgSave(ctx.vg().raw());
                    nvgIntersectScissor(ctx.vg().raw(), x, y, width, height * self.data().collapse_state);
                }
            }

            // Draw the view
            self.draw(ctx.vg(), x, y, width, height);

            if self.data().wireframe_enabled {
                self.draw_wire_frame(ctx, &rect);
            }

            // Reset clipping
            if self.data().collapse_state < 1.0 || self.data().clips_to_bounds {
                unsafe {
                    nvgRestore(ctx.vg().raw());
                }
            }
        }

        unsafe {
            nvgRestore(ctx.vg().raw());
        }
    }

    fn alpha(&self) -> c_float {
        self.data().alpha
    }

    fn a(&self, color: NVGcolor) -> NVGcolor {
        let mut new_color = color.clone();
        new_color.rgba[3] = self.alpha();
        new_color
    }

    fn draw_background(&self, ctx: &FrameContext, rect: &Rect) {
        let x = rect.min_x();
        let y = rect.min_y();
        let width = rect.width();
        let height = rect.height();
        let theme_selected = ctx.theme();

        let vg = ctx.vg().raw();

        match self.data().background {
            ViewBackground::None => {}
            ViewBackground::SideBar => {
                let backdrop_height = style("brls/sidebar/border_height");
                let sidebar_color = theme(theme_selected, "brls/sidebar/background");
                unsafe {
                    // Solid color
                    nvgBeginPath(vg);

                    nvgFillColor(vg, self.a(sidebar_color));
                    nvgRect(vg, x, y + backdrop_height, width, height - backdrop_height * 2.0);
                    nvgFill(vg);

                    //Borders gradient
                    // Top
                    let top_gradient = nvgLinearGradient(vg, x, y + backdrop_height, x, y, self.a(sidebar_color), transparent_color());
                    nvgBeginPath(vg);
                    nvgFillPaint(vg, top_gradient);
                    nvgRect(vg, x, y, width, backdrop_height);
                    nvgFill(vg);

                    // Bottom
                    let bottom_gradient = nvgLinearGradient(vg, x, y + height - backdrop_height, x, y + height, self.a(sidebar_color), transparent_color());
                    nvgBeginPath(vg);
                    nvgFillPaint(vg, bottom_gradient);
                    nvgRect(vg, x, y + height - backdrop_height, width, backdrop_height);
                    nvgFill(vg);
                }
            }
            ViewBackground::BackDrop => {
                let backdrop_color = theme(theme_selected, "brls/backdrop");
                unsafe {
                    nvgFillColor(vg, self.a(backdrop_color));
                    nvgBeginPath(vg);
                    nvgRect(vg, x, y, width, height);
                    nvgFill(vg);
                }
            }
            ViewBackground::ShapeColor => {
                unsafe {
                    nvgFillColor(vg, self.a(self.data().background_color));
                    nvgBeginPath(vg);

                    if self.data().corner_radius > 0.0 {
                        nvgRoundedRect(vg, x, y, width, height, self.data().corner_radius);
                    } else {
                        nvgRect(vg, x, y, width, height);
                    }
                    nvgFill(vg);
                }
            }
            ViewBackground::VerticalLinear => {
                unsafe {
                    let gradient = nvgLinearGradient(vg, x, y, x, y + height, self.a(self.data().background_start_color), self.a(self.data().background_end_color));
                    nvgBeginPath(vg);
                    nvgFillPaint(vg, gradient);
                    let background_radius = &self.data().background_radius;
                    if background_radius.iter().all(|&i| i == 0.0) {
                        nvgRect(vg, x, y, width, height);
                    } else {
                        nvgRoundedRectVarying(vg, x, y, width, height, background_radius[0], background_radius[1], background_radius[2], background_radius[3]);
                    }
                    nvgFill(vg);
                }
            }
        }
    }

    fn draw_border(&self, ctx: &FrameContext, rect: &Rect) {
        let vg = ctx.vg().raw();
        unsafe {
            nvgBeginPath(vg);
            nvgStrokeColor(vg, self.a(self.data().border_color));
            nvgStrokeWidth(vg, self.data().border_thickness);
            nvgRoundedRect(vg, rect.min_x(), rect.min_y(), rect.width(), rect.height(), self.data().corner_radius);
            nvgBeginPath(vg);
        }
    }

    fn draw_shadow(&self, ctx: &FrameContext, rect: &Rect) {
        let mut shadow_width = 0.0f32;
        let mut shadow_feather = 0.0f32;
        let mut shadow_opacity = 0.0f32;
        let mut shadow_offset = 0.0f32;

        match self.data().shadow_type {
            ShadowType::None => {}
            ShadowType::Generic => {
                shadow_width = style("brls/shadow/width");
                shadow_feather = style("brls/shadow/feather");
                shadow_opacity = style("brls/shadow/opacity");
                shadow_offset = style("brls/shadow/offset");
            }
            ShadowType::Custom => {}
        }

        let vg = ctx.vg().raw();

        unsafe {
            let shadow_paint = nvgBoxGradient(
                vg,
                rect.min_x(), rect.min_y() + shadow_width,
                rect.width(), rect.height(),
                self.data().corner_radius * 2.0, shadow_feather,
                nvgRGBA(0, 0, 0, (shadow_opacity * self.data().alpha) as c_uchar), transparent_color());

            nvgBeginPath(vg);
            nvgRect(
                vg,
                rect.min_x() - shadow_offset,
                rect.min_y() - shadow_offset,
                rect.width() + shadow_offset * 2.0,
                rect.height() + shadow_offset * 3.0);
            nvgRoundedRect(vg, rect.min_x(), rect.min_y(), rect.width(), rect.height(), self.data().corner_radius);
            nvgPathWinding(vg, NVGsolidity::NVG_HOLE.bits());
            nvgFillPaint(vg, shadow_paint);
            nvgFill(vg);
        }
    }

    fn draw_line(&self, ctx: &FrameContext, rect: &Rect) {
        let vg = ctx.vg().raw();
        // Don't setup and draw empty nvg path if there is no line to draw
        if self.data().line_top <= 0.0 && self.data().line_left <= 0.0 && self.data().line_bottom <= 0.0 && self.data().line_right <= 0.0 {
            return;
        }

        unsafe {
            nvgBeginPath(vg);
            nvgFillColor(vg, self.a(self.data().line_color));

            if self.data().line_top > 0.0 {
                nvgRect(vg, rect.min_x(), rect.min_y(), rect.size.width, self.data().line_top);
            }

            if self.data().line_right > 0.0 {
                nvgRect(vg, rect.max_x(), rect.min_y(), self.data().line_right, rect.size.height);
            }

            if self.data().line_bottom > 0.0 {
                nvgRect(vg, rect.min_x(), rect.max_y() - self.data().line_bottom, rect.size.width, self.data().line_bottom);
            }

            if self.data().line_left > 0.0 {
                nvgRect(vg, rect.min_x() - self.data().line_left, rect.min_y(), self.data().line_left, rect.size.height);
            }

            nvgFill(vg);
        }
    }

    fn draw_wire_frame(&self, ctx: &FrameContext, rect: &Rect) {
        let vg = ctx.vg().raw();
        unsafe {
            nvgStrokeWidth(vg, 1.0);

            // Outline
            nvgBeginPath(vg);
            nvgStrokeColor(vg, nvgRGB(0, 0, 255));
            nvgRect(vg, rect.min_x(), rect.min_y(), rect.width(), rect.height());
            nvgStroke(vg);

            if self.has_parent()
            {
                // Diagonals
                nvgFillColor(vg, nvgRGB(0, 0, 255));

                nvgBeginPath(vg);
                nvgMoveTo(vg, rect.min_x(), rect.min_y());
                nvgLineTo(vg, rect.max_x(), rect.max_y());
                nvgFill(vg);

                nvgBeginPath(vg);
                nvgMoveTo(vg, rect.max_x(), rect.min_y());
                nvgLineTo(vg, rect.min_x(), rect.max_y());
                nvgFill(vg);
            }

            // Padding
            nvgBeginPath(vg);
            nvgStrokeColor(vg, nvgRGB(0, 255, 0));

            let padding_top = ntz(YGNodeLayoutGetPadding(self.data().yg_node, YGEdgeTop));
            let padding_left = ntz(YGNodeLayoutGetPadding(self.data().yg_node, YGEdgeLeft));
            let padding_bottom = ntz(YGNodeLayoutGetPadding(self.data().yg_node, YGEdgeBottom));
            let padding_right = ntz(YGNodeLayoutGetPadding(self.data().yg_node, YGEdgeRight));

            // Top
            if padding_top > 0.0 {
                nvgRect(vg, rect.min_x(), rect.min_y(), rect.width(), padding_top);
            }

            // Right
            if padding_right > 0.0 {
                nvgRect(vg, rect.max_x() - padding_right, rect.min_y(), padding_right, rect.height());
            }
            // Bottom
            if padding_bottom > 0.0 {
                nvgRect(vg, rect.min_x(), rect.max_y() - padding_bottom, rect.width(), padding_bottom);
            }
            // Left
            if padding_left > 0.0 {
                nvgRect(vg, rect.min_x(), rect.min_y(), padding_left, rect.height());
            }
            nvgStroke(vg);

            // Margins
            nvgBeginPath(vg);
            nvgStrokeColor(vg, nvgRGB(255, 0, 0));

            let margin_top = ntz(YGNodeLayoutGetMargin(self.data().yg_node, YGEdgeTop));
            let margin_left = ntz(YGNodeLayoutGetMargin(self.data().yg_node, YGEdgeLeft));
            let margin_bottom = ntz(YGNodeLayoutGetMargin(self.data().yg_node, YGEdgeBottom));
            let margin_right = ntz(YGNodeLayoutGetMargin(self.data().yg_node, YGEdgeRight));

            // Top
            if margin_top > 0.0 {
                nvgRect(vg, rect.min_x() - margin_left, rect.min_y() - margin_top, rect.width() + margin_left + margin_right, margin_top);
            }
            // Right
            if margin_right > 0.0 {
                nvgRect(vg, rect.max_x(), rect.min_y() - margin_top, margin_right, rect.height() + margin_top + margin_bottom);
            }
            // Bottom
            if margin_bottom > 0.0 {
                nvgRect(vg, rect.min_x() - margin_left, rect.max_y(), rect.width() + margin_left + margin_right, margin_bottom);
            }
            // Left
            if margin_left > 0.0 {
                nvgRect(vg, rect.min_x() - margin_left, rect.min_y() - margin_top, margin_left, rect.height() + margin_top + margin_bottom);
            }
            nvgStroke(vg);
        }
    }

    fn draw_highlight(&self, ctx: &FrameContext, rect: &Rect, alpha: c_float, background: bool) {
        if get_input_type() == InputType::TOUCH {
            return;
        }

        let vg = ctx.vg().raw();

        unsafe {
            nvgSave(vg);

            nvgRestore(vg);
        }
    }

    fn draw_click_animation(&self, ctx: &FrameContext, rect: &Rect) {

    }

    fn draw(
        &self,
        vg: &Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {

    }
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

impl ViewLayout for View {}

impl ViewDraw for View {
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
