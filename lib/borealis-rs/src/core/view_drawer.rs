use nanovg_sys::{nvgBeginPath, nvgBoxGradient, nvgClosePath, NVGcolor, nvgFill, nvgFillColor, nvgFillPaint, nvgLinearGradient, nvgLineTo, nvgMoveTo, nvgPathWinding, nvgRect, nvgRestore, nvgRGB, nvgRGBA, nvgRoundedRect, nvgRoundedRectVarying, nvgSave, NVGsolidity, nvgStroke, nvgStrokeColor, nvgStrokeWidth};
use std::ffi::{c_float, c_uchar};
use yoga_sys::{YGNodeLayoutGetMargin, YGNodeLayoutGetPadding};
use yoga_sys::YGEdge::{YGEdgeBottom, YGEdgeLeft, YGEdgeRight, YGEdgeTop};
use nanovg::Context;
use crate::core::application::{get_input_type, InputType};
use crate::core::frame_context::FrameContext;
use crate::core::geometry::Rect;
use crate::core::style::style;
use crate::core::theme::{theme, transparent_color};
use crate::core::view_base;
use crate::core::view_base::{ShadowType, View, ViewBackground, ViewBase, Visibility};
use crate::core::view_layout::ViewLayout;

pub trait ViewDrawer: ViewLayout {
    fn frame(&self, ctx: &FrameContext) {
        if self.data().visibility != Visibility::Visible {
            return;
        }

        unsafe {
            nvgSave(ctx.vg().raw());
        }

        let rect = self.rect();
        // trace!("rect: {:?}", rect);
        let x = rect.min_x();
        let y = rect.min_y();
        let width = rect.width();
        let height = rect.height();

        if self.data().alpha > 0.0 {
            // Draw background
            self.draw_background(ctx, &rect);

            // // Draw shadow
            // if self.data().shadow_type != ShadowType::None
            //     && (self.data().show_shadow || get_input_type() == InputType::TOUCH)
            // {
            //     self.draw_shadow(ctx, &rect);
            // }
            //
            // // Draw border
            // if self.data().border_thickness > 0.0 {
            //     self.draw_border(ctx, &rect);
            // }
            //
            self.draw_line(ctx, &rect);
            //
            // // Draw highlight background
            // if self.data().highlight_alpha > 0.0
            //     && !self.data().hide_highlight_background
            //     && !self.data().hide_highlight
            // {
            //     self.draw_highlight(ctx, &rect, self.alpha(), true);
            // }
            //
            // // Draw click animation
            // if self.data().click_alpha > 0.0 {
            //     self.draw_click_animation(ctx, &rect);
            // }
            //
            // // Collapse clipping
            // if self.data().collapse_state < 1.0 || self.data().clips_to_bounds {
            //     unsafe {
            //         nvgSave(ctx.vg().raw());
            //         nvgIntersectScissor(
            //             ctx.vg().raw(),
            //             x,
            //             y,
            //             width,
            //             height * self.data().collapse_state,
            //         );
            //     }
            // }
            //
            // // Draw the view
            // self.draw(ctx.vg(), x, y, width, height);
            //
            // if self.data().wireframe_enabled {
            //     self.draw_wire_frame(ctx, &rect);
            // }
            //
            // // Reset clipping
            // if self.data().collapse_state < 1.0 || self.data().clips_to_bounds {
            //     unsafe {
            //         nvgRestore(ctx.vg().raw());
            //     }
            // }
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
                let backdrop_color = theme(theme_selected, "brls/backdrop");
                unsafe {
                    nvgFillColor(vg, self.a(backdrop_color));
                    nvgBeginPath(vg);
                    nvgRect(vg, x, y, width, height);
                    nvgFill(vg);
                }
            }
            ViewBackground::ShapeColor => unsafe {
                nvgFillColor(vg, self.a(self.data().background_color));
                nvgBeginPath(vg);

                if self.data().corner_radius > 0.0 {
                    nvgRoundedRect(vg, x, y, width, height, self.data().corner_radius);
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
                    self.a(self.data().background_start_color),
                    self.a(self.data().background_end_color),
                );
                nvgBeginPath(vg);
                nvgFillPaint(vg, gradient);
                let background_radius = &self.data().background_radius;
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

    fn draw_border(&self, ctx: &FrameContext, rect: &Rect) {
        let vg = ctx.vg().raw();
        unsafe {
            nvgBeginPath(vg);
            nvgStrokeColor(vg, self.a(self.data().border_color));
            nvgStrokeWidth(vg, self.data().border_thickness);
            nvgRoundedRect(
                vg,
                rect.min_x(),
                rect.min_y(),
                rect.width(),
                rect.height(),
                self.data().corner_radius,
            );
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
                rect.min_x(),
                rect.min_y() + shadow_width,
                rect.width(),
                rect.height(),
                self.data().corner_radius * 2.0,
                shadow_feather,
                nvgRGBA(0, 0, 0, (shadow_opacity * self.data().alpha) as c_uchar),
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
                self.data().corner_radius,
            );
            nvgPathWinding(vg, NVGsolidity::NVG_HOLE.bits());
            nvgFillPaint(vg, shadow_paint);
            nvgFill(vg);
        }
    }

    fn draw_line(&self, ctx: &FrameContext, rect: &Rect) {
        let vg = ctx.vg().raw();
        // Don't setup and draw empty nvg path if there is no line to draw
        if self.data().line_top <= 0.0
            && self.data().line_left <= 0.0
            && self.data().line_bottom <= 0.0
            && self.data().line_right <= 0.0
        {
            return;
        }

        unsafe {

            if self.data().line_top > 0.0 {
                nvgBeginPath(vg);
                nvgRect(
                    vg,
                    rect.min_x(),
                    rect.min_y(),
                    rect.size.width,
                    self.data().line_top,
                );
                nvgClosePath(vg);
                nvgFillColor(vg, self.a(self.data().line_color));
                nvgFill(vg);
            }

            if self.data().line_right > 0.0 {
                nvgBeginPath(vg);
                nvgRect(
                    vg,
                    rect.max_x(),
                    rect.min_y(),
                    self.data().line_right,
                    rect.size.height,
                );
                nvgClosePath(vg);
                nvgFillColor(vg, self.a(self.data().line_color));
                nvgFill(vg);
            }

            if self.data().line_bottom > 0.0 {
                nvgBeginPath(vg);
                nvgRect(
                    vg,
                    rect.min_x(),
                    rect.max_y() - self.data().line_bottom,
                    rect.size.width,
                    self.data().line_bottom,
                );
                nvgClosePath(vg);
                nvgFillColor(vg, self.a(self.data().line_color));
                nvgFill(vg);
            }

            if self.data().line_left > 0.0 {
                nvgBeginPath(vg);
                nvgRect(
                    vg,
                    rect.min_x() - self.data().line_left,
                    rect.min_y(),
                    self.data().line_left,
                    rect.size.height,
                );
                nvgClosePath(vg);
                nvgFillColor(vg, self.a(self.data().line_color));
                nvgFill(vg);
            }
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

            let padding_top = view_base::ntz(YGNodeLayoutGetPadding(self.data().yg_node, YGEdgeTop));
            let padding_left = view_base::ntz(YGNodeLayoutGetPadding(self.data().yg_node, YGEdgeLeft));
            let padding_bottom = view_base::ntz(YGNodeLayoutGetPadding(self.data().yg_node, YGEdgeBottom));
            let padding_right = view_base::ntz(YGNodeLayoutGetPadding(self.data().yg_node, YGEdgeRight));

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

            let margin_top = view_base::ntz(YGNodeLayoutGetMargin(self.data().yg_node, YGEdgeTop));
            let margin_left = view_base::ntz(YGNodeLayoutGetMargin(self.data().yg_node, YGEdgeLeft));
            let margin_bottom = view_base::ntz(YGNodeLayoutGetMargin(self.data().yg_node, YGEdgeBottom));
            let margin_right = view_base::ntz(YGNodeLayoutGetMargin(self.data().yg_node, YGEdgeRight));

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

    fn draw_click_animation(&self, ctx: &FrameContext, rect: &Rect) {}

    fn draw(&self, vg: &Context, x: f32, y: f32, width: f32, height: f32) {}
}

impl ViewDrawer for View {}