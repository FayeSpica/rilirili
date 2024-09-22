use crate::core::animation::Animating;
use crate::core::application::{get_input_type, InputType};
use crate::core::frame_context::FrameContext;
use crate::core::geometry::Rect;
use crate::core::style::style;
use crate::core::theme::{theme, transparent_color};
use crate::core::view_base;
use crate::core::view_base::{
    ShadowType, TransitionAnimation, View, ViewBackground, ViewBase, Visibility,
};
use crate::core::view_box::BoxTrait;
use crate::core::view_layout::ViewLayout;
use nanovg_sys::{nvgBeginPath, nvgBoxGradient, nvgClosePath, nvgFill, nvgFillColor, nvgFillPaint, nvgIntersectScissor, nvgLineTo, nvgLinearGradient, nvgMoveTo, nvgPathWinding, nvgRGB, nvgRGBA, nvgRGBAf, nvgRect, nvgResetScissor, nvgRestore, nvgRoundedRect, nvgRoundedRectVarying, nvgSave, nvgStroke, nvgStrokeColor, nvgStrokeWidth, NVGcolor, NVGsolidity, NVGpaint};
use std::ffi::{c_float, c_uchar};
use yoga_sys::YGEdge::{YGEdgeBottom, YGEdgeLeft, YGEdgeRight, YGEdgeTop};
use yoga_sys::{YGNodeLayoutGetMargin, YGNodeLayoutGetPadding};

pub trait ViewTrait: ViewDrawer {}

pub trait ViewDrawer: ViewLayout {
    /**
     * Called each frame
     * Do not override it to draw your view,
     * override draw() instead
     */
    fn frame(&self, ctx: &FrameContext) {
        if self.view_data().borrow().visibility != Visibility::Visible {
            return;
        }

        unsafe {
            nvgSave(ctx.context);
        }

        let rect = self.rect();
        let x = rect.min_x();
        let y = rect.min_y();
        let width = rect.width();
        let height = rect.height();

        if self.view_data().borrow().alpha.current_value > 0.0 {
            // Draw background
            self.draw_background(ctx, &rect);

            // Draw shadow
            if self.view_data().borrow().shadow_type != ShadowType::None
                && (self.view_data().borrow().show_shadow || get_input_type() == InputType::TOUCH)
            {
                self.draw_shadow(ctx, &rect);
            }

            // Draw border
            if self.view_data().borrow().border_thickness > 0.0 {
                self.draw_border(ctx, &rect);
            }

            self.draw_line(ctx, &rect);

            // Draw highlight background
            if self.view_data().borrow().highlight_alpha.current_value > 0.0
                && !self.view_data().borrow().hide_highlight_background
                && !self.view_data().borrow().hide_highlight
            {
                self.draw_highlight(ctx, &rect, self.view_data().borrow().highlight_alpha.value(), true);
            }

            // Draw click animation
            if self.view_data().borrow().click_alpha.current_value > 0.0 {
                self.draw_click_animation(ctx, &rect);
            }

            // Collapse clipping
            if self.view_data().borrow().collapse_state.current_value < 1.0 || self.view_data().borrow().clips_to_bounds {
                unsafe {
                    nvgSave(ctx.context);
                    nvgIntersectScissor(
                        ctx.context,
                        x,
                        y,
                        width,
                        height * self.view_data().borrow().collapse_state.current_value,
                    );
                }
            }

            // Draw the view
            self.draw(ctx, x, y, width, height);

            if self.view_data().borrow().wireframe_enabled {
                self.draw_wire_frame(ctx, &rect);
            }

            // Reset clipping
            if self.view_data().borrow().collapse_state.current_value < 1.0 || self.view_data().borrow().clips_to_bounds {
                unsafe {
                    nvgRestore(ctx.context);
                }
            }
        }

        unsafe {
            nvgRestore(ctx.context);
        }
    }

    /**
     * Called each frame
     */
    fn frame_highlight(&self, ctx: &FrameContext) {
        todo!()
    }

    /**
     * Called by frame() to draw the view onscreen.
     * Views should not draw outside of their bounds (they
     * may be clipped if they do so).
     */
    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        trace!("default draw");
    }

    /**
     * Called when the show() animation (fade in)
     * ends
     */
    fn on_show_animation_end(&self) {}

    /**
     * Shows the view with a fade in animation.
     */
    fn show(&self, cb: Box<dyn Fn()>) {
        self.show_animated(
            cb,
            true,
            self.show_animation_duration(TransitionAnimation::Fade),
        );
    }

    /**
     * Shows the view with a fade in animation, or no animation at all.
     */
    fn show_animated(&self, cb: Box<dyn Fn()>, animate: bool, animation_duration: f32) {
        if !self.view_data().borrow().hidden {
            self.on_show_animation_end();
            cb();
            return;
        }

        debug!("Showing {}", self.describe());

        self.view_data().borrow_mut().hidden = false;

        self.view_data().borrow_mut().fade_in = true;

        if animate {
        } else {
            self.view_data().borrow_mut().alpha.current_value = 1.0;
            self.view_data().borrow_mut().fade_in = false;
            self.on_show_animation_end();
            cb();
        }
    }

    /**
     * Returns the duration of the view show / hide animation.
     */
    fn show_animation_duration(&self, animation: TransitionAnimation) -> f32 {
        if animation == TransitionAnimation::SlideLeft
            || animation == TransitionAnimation::SlideRight
        {
            panic!("Slide animation is not supported on views");
        }

        style("brls/animations/show")
    }

    /**
     * Hides the view with a fade out animation.
     */
    fn hide(&mut self, cb: Box<dyn Fn()>) {
        self.hide_animated(
            cb,
            true,
            self.show_animation_duration(TransitionAnimation::Fade),
        );
    }

    /**
     * Hides the view with a fade out animation, or no animation at all.
     */
    fn hide_animated(&self, cb: Box<dyn Fn()>, animate: bool, animation_duration: f32) {
        if self.view_data().borrow().hidden {
            cb();
            return;
        }

        debug!("Hiding {}", self.describe());

        self.view_data().borrow_mut().hidden = true;
        self.view_data().borrow_mut().fade_in = false;

        if animate {
        } else {
            self.view_data().borrow_mut().alpha.current_value = 0.0;
            cb();
        }
    }

    fn alpha(&self) -> c_float {
        self.view_data().borrow().alpha.current_value
    }

    fn a(&self, color: NVGcolor) -> NVGcolor {
        let mut new_color = color.clone();
        new_color.rgba[3] *= self.alpha();
        new_color
    }

    fn a_paint(&self, paint: NVGpaint) -> NVGpaint {
        let mut new_paint = paint.clone();
        new_paint.innerColor.rgba[3] *= self.alpha();
        new_paint.outerColor.rgba[3] *= self.alpha();
        new_paint
    }

    fn draw_background(&self, ctx: &FrameContext, rect: &Rect) {
        let x = rect.min_x();
        let y = rect.min_y();
        let width = rect.width();
        let height = rect.height();

        let vg = ctx.context;

        match self.view_data().borrow().background {
            ViewBackground::None => {}
            ViewBackground::SideBar => {
                let backdrop_height = style("brls/sidebar/border_height");
                let sidebar_color = theme("brls/sidebar/background");
                unsafe {
                    // Solid color
                    nvgBeginPath(vg);

                    nvgFillColor(vg, self.a(sidebar_color));
                    nvgRect(
                        vg,
                        x,
                        y + backdrop_height,
                        width,
                        height - backdrop_height * 2.0,
                    );
                    nvgFill(vg);

                    //Borders gradient
                    // Top
                    let top_gradient = nvgLinearGradient(
                        vg,
                        x,
                        y + backdrop_height,
                        x,
                        y,
                        self.a(sidebar_color),
                        transparent_color(),
                    );
                    nvgBeginPath(vg);
                    nvgFillPaint(vg, top_gradient);
                    nvgRect(vg, x, y, width, backdrop_height);
                    nvgFill(vg);

                    // Bottom
                    let bottom_gradient = nvgLinearGradient(
                        vg,
                        x,
                        y + height - backdrop_height,
                        x,
                        y + height,
                        self.a(sidebar_color),
                        transparent_color(),
                    );
                    nvgBeginPath(vg);
                    nvgFillPaint(vg, bottom_gradient);
                    nvgRect(vg, x, y + height - backdrop_height, width, backdrop_height);
                    nvgFill(vg);
                }
            }
            ViewBackground::BackDrop => {
                let backdrop_color = theme("brls/backdrop");
                unsafe {
                    nvgFillColor(vg, self.a(backdrop_color));
                    nvgBeginPath(vg);
                    nvgRect(vg, x, y, width, height);
                    nvgFill(vg);
                }
            }
            ViewBackground::ShapeColor => unsafe {
                nvgFillColor(vg, self.a(self.view_data().borrow().background_color));
                nvgBeginPath(vg);

                if self.view_data().borrow().corner_radius > 0.0 {
                    nvgRoundedRect(vg, x, y, width, height, self.view_data().borrow().corner_radius);
                } else {
                    nvgRect(vg, x, y, width, height);
                }
                nvgFill(vg);
            },
            ViewBackground::VerticalLinear => unsafe {
                let gradient = nvgLinearGradient(
                    vg,
                    x,
                    y,
                    x,
                    y + height,
                    self.a(self.view_data().borrow().background_start_color),
                    self.a(self.view_data().borrow().background_end_color),
                );
                nvgBeginPath(vg);
                nvgFillPaint(vg, gradient);
                let background_radius = &self.view_data().borrow().background_radius;
                if background_radius.iter().all(|&i| i == 0.0) {
                    nvgRect(vg, x, y, width, height);
                } else {
                    nvgRoundedRectVarying(
                        vg,
                        x,
                        y,
                        width,
                        height,
                        background_radius[0],
                        background_radius[1],
                        background_radius[2],
                        background_radius[3],
                    );
                }
                nvgFill(vg);
            },
        }
    }

    fn draw_shadow(&self, ctx: &FrameContext, rect: &Rect) {
        let mut shadow_width = 0.0f32;
        let mut shadow_feather = 0.0f32;
        let mut shadow_opacity = 0.0f32;
        let mut shadow_offset = 0.0f32;

        match self.view_data().borrow().shadow_type {
            ShadowType::None => {}
            ShadowType::Generic => {
                shadow_width = style("brls/shadow/width");
                shadow_feather = style("brls/shadow/feather");
                shadow_opacity = style("brls/shadow/opacity");
                shadow_offset = style("brls/shadow/offset");
            }
            ShadowType::Custom => {}
        }

        let vg = ctx.context;

        unsafe {
            let shadow_paint = nvgBoxGradient(
                vg,
                rect.min_x(),
                rect.min_y() + shadow_width,
                rect.width(),
                rect.height(),
                self.view_data().borrow().corner_radius * 2.0,
                shadow_feather,
                nvgRGBA(
                    0,
                    0,
                    0,
                    (shadow_opacity * self.view_data().borrow().alpha.current_value) as c_uchar,
                ),
                transparent_color(),
            );

            nvgBeginPath(vg);
            nvgRect(
                vg,
                rect.min_x() - shadow_offset,
                rect.min_y() - shadow_offset,
                rect.width() + shadow_offset * 2.0,
                rect.height() + shadow_offset * 3.0,
            );
            nvgRoundedRect(
                vg,
                rect.min_x(),
                rect.min_y(),
                rect.width(),
                rect.height(),
                self.view_data().borrow().corner_radius,
            );
            nvgPathWinding(vg, NVGsolidity::NVG_HOLE.bits());
            nvgFillPaint(vg, shadow_paint);
            nvgFill(vg);
        }
    }

    fn draw_border(&self, ctx: &FrameContext, rect: &Rect) {
        let vg = ctx.context;
        unsafe {
            nvgBeginPath(vg);
            nvgStrokeColor(vg, self.a(self.view_data().borrow().border_color));
            nvgStrokeWidth(vg, self.view_data().borrow().border_thickness);
            nvgRoundedRect(
                vg,
                rect.min_x(),
                rect.min_y(),
                rect.width(),
                rect.height(),
                self.view_data().borrow().corner_radius,
            );
            nvgBeginPath(vg);
        }
    }

    fn draw_highlight(&self, ctx: &FrameContext, rect: &Rect, alpha: c_float, background: bool) {
        if get_input_type() == InputType::TOUCH {
            return;
        }

        let vg = ctx.context;

        unsafe {
            nvgSave(vg);
            nvgResetScissor(vg);
        }

        let padding = self.view_data().borrow().highlight_padding;
        let corner_radius = self.view_data().borrow().highlight_corner_radius;
        let stroke_width = style("brls/highlight/stroke_width");

        let x = self.x() - padding - stroke_width / 2.0;
        let y = self.y() - padding - stroke_width / 2.0;
        let width = self.width() + padding * 2.0 + stroke_width;
        let height = self.height() + padding * 2.0 + stroke_width;

        // Draw
        if background {
            // Background
            let highlight_background_color = theme("brls/highlight/background");
            unsafe {
                nvgFillColor(
                    vg,
                    nvgRGBAf(
                        highlight_background_color.rgba[0],
                        highlight_background_color.rgba[1],
                        highlight_background_color.rgba[2],
                        self.view_data().borrow().highlight_alpha.value(),
                    ),
                );
                nvgBeginPath(vg);
                nvgRoundedRect(vg, x, y, width, height, corner_radius);
                nvgFill(vg);
            }
        } else {
            // Border
            let shadowOffset = style("brls/highlight/shadow_offset");
            todo!()
        }

        unsafe {
            nvgRestore(vg);
        }
    }

    fn draw_click_animation(&self, ctx: &FrameContext, rect: &Rect) {}

    fn draw_wire_frame(&self, ctx: &FrameContext, rect: &Rect) {
        let vg = ctx.context;
        unsafe {
            nvgStrokeWidth(vg, 1.0);

            // Outline
            nvgBeginPath(vg);
            nvgStrokeColor(vg, nvgRGB(0, 0, 255));
            nvgRect(vg, rect.min_x(), rect.min_y(), rect.width(), rect.height());
            nvgStroke(vg);

            if self.has_parent() {
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

            let padding_top =
                view_base::ntz(YGNodeLayoutGetPadding(self.view_data().borrow().yg_node, YGEdgeTop));
            let padding_left =
                view_base::ntz(YGNodeLayoutGetPadding(self.view_data().borrow().yg_node, YGEdgeLeft));
            let padding_bottom =
                view_base::ntz(YGNodeLayoutGetPadding(self.view_data().borrow().yg_node, YGEdgeBottom));
            let padding_right =
                view_base::ntz(YGNodeLayoutGetPadding(self.view_data().borrow().yg_node, YGEdgeRight));

            // Top
            if padding_top > 0.0 {
                nvgRect(vg, rect.min_x(), rect.min_y(), rect.width(), padding_top);
            }

            // Right
            if padding_right > 0.0 {
                nvgRect(
                    vg,
                    rect.max_x() - padding_right,
                    rect.min_y(),
                    padding_right,
                    rect.height(),
                );
            }
            // Bottom
            if padding_bottom > 0.0 {
                nvgRect(
                    vg,
                    rect.min_x(),
                    rect.max_y() - padding_bottom,
                    rect.width(),
                    padding_bottom,
                );
            }
            // Left
            if padding_left > 0.0 {
                nvgRect(vg, rect.min_x(), rect.min_y(), padding_left, rect.height());
            }
            nvgStroke(vg);

            // Margins
            nvgBeginPath(vg);
            nvgStrokeColor(vg, nvgRGB(255, 0, 0));

            let margin_top = view_base::ntz(YGNodeLayoutGetMargin(self.view_data().borrow().yg_node, YGEdgeTop));
            let margin_left =
                view_base::ntz(YGNodeLayoutGetMargin(self.view_data().borrow().yg_node, YGEdgeLeft));
            let margin_bottom =
                view_base::ntz(YGNodeLayoutGetMargin(self.view_data().borrow().yg_node, YGEdgeBottom));
            let margin_right =
                view_base::ntz(YGNodeLayoutGetMargin(self.view_data().borrow().yg_node, YGEdgeRight));

            // Top
            if margin_top > 0.0 {
                nvgRect(
                    vg,
                    rect.min_x() - margin_left,
                    rect.min_y() - margin_top,
                    rect.width() + margin_left + margin_right,
                    margin_top,
                );
            }
            // Right
            if margin_right > 0.0 {
                nvgRect(
                    vg,
                    rect.max_x(),
                    rect.min_y() - margin_top,
                    margin_right,
                    rect.height() + margin_top + margin_bottom,
                );
            }
            // Bottom
            if margin_bottom > 0.0 {
                nvgRect(
                    vg,
                    rect.min_x() - margin_left,
                    rect.max_y(),
                    rect.width() + margin_left + margin_right,
                    margin_bottom,
                );
            }
            // Left
            if margin_left > 0.0 {
                nvgRect(
                    vg,
                    rect.min_x() - margin_left,
                    rect.min_y() - margin_top,
                    margin_left,
                    rect.height() + margin_top + margin_bottom,
                );
            }
            nvgStroke(vg);
        }
    }

    fn draw_line(&self, ctx: &FrameContext, rect: &Rect) {
        let vg = ctx.context;
        // Don't setup and draw empty nvg path if there is no line to draw
        if self.view_data().borrow().line_top <= 0.0
            && self.view_data().borrow().line_left <= 0.0
            && self.view_data().borrow().line_bottom <= 0.0
            && self.view_data().borrow().line_right <= 0.0
        {
            return;
        }

        unsafe {
            if self.view_data().borrow().line_top > 0.0 {
                nvgBeginPath(vg);
                nvgRect(
                    vg,
                    rect.min_x(),
                    rect.min_y(),
                    rect.size.width,
                    self.view_data().borrow().line_top,
                );
                nvgClosePath(vg);
                nvgFillColor(vg, self.a(self.view_data().borrow().line_color));
                nvgFill(vg);
            }

            if self.view_data().borrow().line_right > 0.0 {
                nvgBeginPath(vg);
                nvgRect(
                    vg,
                    rect.max_x(),
                    rect.min_y(),
                    self.view_data().borrow().line_right,
                    rect.size.height,
                );
                nvgClosePath(vg);
                nvgFillColor(vg, self.a(self.view_data().borrow().line_color));
                nvgFill(vg);
            }

            if self.view_data().borrow().line_bottom > 0.0 {
                nvgBeginPath(vg);
                nvgRect(
                    vg,
                    rect.min_x(),
                    rect.max_y() - self.view_data().borrow().line_bottom,
                    rect.size.width,
                    self.view_data().borrow().line_bottom,
                );
                nvgClosePath(vg);
                nvgFillColor(vg, self.a(self.view_data().borrow().line_color));
                nvgFill(vg);
            }

            if self.view_data().borrow().line_left > 0.0 {
                nvgBeginPath(vg);
                nvgRect(
                    vg,
                    rect.min_x() - self.view_data().borrow().line_left,
                    rect.min_y(),
                    self.view_data().borrow().line_left,
                    rect.size.height,
                );
                nvgClosePath(vg);
                nvgFillColor(vg, self.a(self.view_data().borrow().line_color));
                nvgFill(vg);
            }
        }
    }
}
