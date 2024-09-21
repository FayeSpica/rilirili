use crate::core::application::ViewCreatorRegistry;
use crate::core::geometry::{Point, Rect, Size};
use crate::core::theme::{AUTO, nvg_rgb, nvg_rgba, theme, YG_UNDEFINED};
use crate::core::view_base::{AlignSelf, FocusDirection, ntz, PositionType, ShadowType, View, ViewBackground, ViewBase, Visibility};
use crate::core::view_style::ViewStyle;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::str::FromStr;
use yoga_sys::YGAlign::{
    YGAlignAuto, YGAlignBaseline, YGAlignCenter, YGAlignFlexEnd, YGAlignFlexStart,
    YGAlignSpaceAround, YGAlignSpaceBetween, YGAlignStretch,
};
use yoga_sys::YGPositionType::{YGPositionTypeAbsolute, YGPositionTypeRelative};
use yoga_sys::{YGDirection, YGEdge, YGNodeCalculateLayout, YGNodeLayoutGetHeight, YGNodeLayoutGetLeft, YGNodeLayoutGetTop, YGNodeLayoutGetWidth, YGNodeStyleGetMargin, YGNodeStyleSetAlignSelf, YGNodeStyleSetDisplay, YGNodeStyleSetFlexGrow, YGNodeStyleSetFlexShrink, YGNodeStyleSetHeight, YGNodeStyleSetHeightAuto, YGNodeStyleSetHeightPercent, YGNodeStyleSetMargin, YGNodeStyleSetMarginAuto, YGNodeStyleSetMaxHeight, YGNodeStyleSetMaxHeightPercent, YGNodeStyleSetMaxWidth, YGNodeStyleSetMaxWidthPercent, YGNodeStyleSetMinHeight, YGNodeStyleSetMinHeightPercent, YGNodeStyleSetMinWidth, YGNodeStyleSetMinWidthPercent, YGNodeStyleSetPosition, YGNodeStyleSetPositionPercent, YGNodeStyleSetPositionType, YGNodeStyleSetWidth, YGNodeStyleSetWidthAuto, YGNodeStyleSetWidthPercent};
use yoga_sys::YGDisplay::{YGDisplayFlex, YGDisplayNone};
use yoga_sys::YGEdge::{YGEdgeBottom, YGEdgeLeft, YGEdgeRight, YGEdgeTop};
use crate::core::attribute::{AutoAttributeHandler, BoolAttributeHandler, ColorAttributeHandler, FloatAttributeHandler, StringAttributeHandler};
use crate::core::style::{hex_to_rgb, hex_to_rgba, style};
use crate::core::view_creator::get_file_path_xml_attribute_value;

pub trait ViewLayout: ViewStyle {
    fn shake_highlight(&self, direction: FocusDirection) {
        todo!()
    }

    fn rect(&self) -> Rect {
        return Rect::new(
            Point::new(self.x(), self.y()),
            Size::new(self.width(), self.height()),
        );
    }

    fn x(&self) -> f32 {
        return unsafe { YGNodeLayoutGetLeft(self.view_data().borrow().yg_node) };
    }

    fn y(&self) -> f32 {
        return unsafe { YGNodeLayoutGetTop(self.view_data().borrow().yg_node) };
    }

    fn local_rect(&self) -> Rect {
        return Rect::new(
            Point::new(self.local_x(), self.local_y()),
            Size::new(self.width(), self.height()),
        );
    }

    fn local_x(&self) -> f32 {
        return unsafe { YGNodeLayoutGetLeft(self.view_data().borrow().yg_node) };
    }

    fn local_y(&self) -> f32 {
        return unsafe { YGNodeLayoutGetTop(self.view_data().borrow().yg_node) };
    }

    fn width(&self) -> f32 {
        return unsafe { YGNodeLayoutGetWidth(self.view_data().borrow().yg_node) };
    }

    fn height(&self) -> f32 {
        return unsafe { YGNodeLayoutGetHeight(self.view_data().borrow().yg_node) };
    }

    fn height_include_collapse(&self) -> f32 {
        todo!()
    }

    /**
     * Triggers a layout of the whole view tree. Must be called
     * after a yoga node property is changed.
     *
     * Only methods that change yoga nodes properties should
     * call this method.
     */
    fn invalidate(&self) {
        if self.has_parent() {
            self.parent().as_ref().unwrap().borrow().invalidate();
        } else {
            unsafe {
                YGNodeCalculateLayout(
                    self.view_data().borrow().yg_node,
                    f32::NAN,
                    f32::NAN,
                    YGDirection::YGDirectionLTR,
                )
            }
        }
    }

    /**
     * Called when a layout pass ends on that view.
     */
    fn on_layout(&self) {}

    /**
     * Returns the view with the corresponding id in the view or its children,
     * or nullptr if it hasn't been found.
     *
     * Research is done recursively by traversing the tree starting from this view.
     * This view's parents are not traversed.
     */
    fn get_view(&self, id: &str) -> Rc<RefCell<View>> {
        todo!()
    }

    // -----------------------------------------------------------
    // Flex layout properties
    // -----------------------------------------------------------

    /**
     * Sets the preferred width of the view. Use brls::View::AUTO
     * to have the layout automatically resize the view.
     *
     * If set to anything else than AUTO, the view is guaranteed
     * to never shrink below the given width.
     */
    fn set_width(&self, width: f32) {
        unsafe {
            YGNodeStyleSetMinWidthPercent(self.view_data().borrow().yg_node, 0.0);
            YGNodeStyleSetWidth(self.view_data().borrow().yg_node, width);
            YGNodeStyleSetMinWidth(self.view_data().borrow().yg_node, width);
        }
        // self.invalidate();
    }

    /**
     * Sets the preferred height of the view. Use brls::View::AUTO
     * to have the layout automatically resize the view.
     *
     * If set to anything else than AUTO, the view is guaranteed
     * to never shrink below the given height.
     */
    fn set_height(&self, height: f32) {
        unsafe {
            YGNodeStyleSetMinHeightPercent(self.view_data().borrow().yg_node, 0.0);
            YGNodeStyleSetHeight(self.view_data().borrow().yg_node, height);
            YGNodeStyleSetMinHeight(self.view_data().borrow().yg_node, height);
        }
        // self.invalidate();
    }

    /**
     * Shortcut to setWidth + setHeight.
     *
     * Only does one layout pass instead of two when using the two methods separately.
     */
    fn set_dimensions(&self, width: f32, height: f32) {
        warn!("set_dimensions({}, {})", width, height);
        unsafe {
            YGNodeStyleSetMinWidthPercent(self.view_data().borrow().yg_node, 0.0);
            YGNodeStyleSetMinHeightPercent(self.view_data().borrow().yg_node, 0.0);

            match width == AUTO {
                true => {
                    YGNodeStyleSetWidthAuto(self.view_data().borrow().yg_node);
                    YGNodeStyleSetMinWidth(self.view_data().borrow().yg_node, YG_UNDEFINED);
                }
                false => {
                    YGNodeStyleSetWidth(self.view_data().borrow().yg_node, width);
                    YGNodeStyleSetMinWidth(self.view_data().borrow().yg_node, width);
                }
            }

            match height == AUTO {
                true => {
                    YGNodeStyleSetHeightAuto(self.view_data().borrow().yg_node);
                    YGNodeStyleSetMinHeight(self.view_data().borrow().yg_node, YG_UNDEFINED);
                }
                false => {
                    YGNodeStyleSetHeight(self.view_data().borrow().yg_node, height);
                    YGNodeStyleSetMinHeight(self.view_data().borrow().yg_node, height);
                }
            }
        }

        self.invalidate();
    }

    /**
     * Sets the preferred width of the view in percentage of
     * the parent view width. Between 0.0f and 100.0f.
     */
    fn set_width_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetWidthPercent(self.view_data().borrow().yg_node, percentage);
            YGNodeStyleSetMinWidthPercent(self.view_data().borrow().yg_node, percentage);
        }
        self.invalidate();
    }

    /**
     * Sets the preferred height of the view in percentage of
     * the parent view height. Between 0.0f and 100.0f.
     */
    fn set_height_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetHeightPercent(self.view_data().borrow().yg_node, percentage);
            YGNodeStyleSetMinHeightPercent(self.view_data().borrow().yg_node, percentage);
        }
        self.invalidate();
    }

    /**
     * Sets the maximum width of the view, in pixels.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be larger than the given max width.
     *
     * Use View::AUTO to disable the max width constraint.
     */
    fn set_max_width(&self, max_width: f32) {
        unsafe {
            if max_width == AUTO {
                YGNodeStyleSetMaxWidth(self.view_data().borrow().yg_node, YG_UNDEFINED);
            } else {
                YGNodeStyleSetMaxWidth(self.view_data().borrow().yg_node, max_width);
            }
        }
        self.invalidate();
    }

    /**
     * Sets the maximum height of the view, in pixels.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be larger than the given max height.
     *
     * Use View::AUTO to disable the max height constraint.
     */
    fn set_max_height(&self, max_height: f32) {
        unsafe {
            if max_height == AUTO {
                YGNodeStyleSetMaxHeight(self.view_data().borrow().yg_node, YG_UNDEFINED);
            } else {
                YGNodeStyleSetMaxHeight(self.view_data().borrow().yg_node, max_height);
            }
        }
        self.invalidate();
    }

    /**
     * Sets the maximum width of the view, in parent width percentage.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be larger than the given max width.
     *
     * Use View::AUTO to disable the max width constraint.
     */
    fn set_max_width_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetMaxWidthPercent(self.view_data().borrow().yg_node, percentage);
        }
        self.invalidate();
    }

    /**
     * Sets the maximum height of the view, in parent height percentage.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be larger than the given max height.
     *
     * Use View::AUTO to disable the max height constraint.
     */
    fn set_max_height_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetMaxHeightPercent(self.view_data().borrow().yg_node, percentage);
        }
        self.invalidate();
    }

    /**
     * Sets the grow factor of the view, aka the percentage
     * of remaining space to give this view, in the containing box axis.
     * Opposite of shrink.
     * Default is 0.0f;
     */
    fn set_grow(&self, grow: f32) {
        unsafe {
            YGNodeStyleSetFlexGrow(self.view_data().borrow().yg_node, grow);
        }
        self.invalidate();
    }

    /**
     * Sets the shrink factor of the view, aka the percentage of space
     * the view is allowed to shrink for if there is not enough space for everyone
     * in the contaning box axis. Opposite of grow.
     * Default is 1.0f;
     */
    fn set_shrink(&self, shrink: f32) {
        unsafe {
            YGNodeStyleSetFlexShrink(self.view_data().borrow().yg_node, shrink);
        }
        self.invalidate();
    }

    /**
     * Sets the margin of the view, aka the space that separates
     * this view and the surrounding ones in all 4 directions.
     *
     * Use brls::View::AUTO to have the layout automatically select the
     * margin.
     *
     * Only works with views that have parents - top level views that are pushed
     * on the stack don't have parents.
     *
     * Only does one layout pass instead of four when using the four methods separately.
     */
    fn set_margins(&self, top: f32, right: f32, bottom: f32, left: f32) {
        unsafe {
            if top == AUTO {
                YGNodeStyleSetMarginAuto(self.view_data().borrow().yg_node, YGEdgeTop);
            } else {
                YGNodeStyleSetMargin(self.view_data().borrow().yg_node, YGEdgeTop, top);
            }

            if right == AUTO {
                YGNodeStyleSetMarginAuto(self.view_data().borrow().yg_node, YGEdgeRight);
            } else {
                YGNodeStyleSetMargin(self.view_data().borrow().yg_node, YGEdgeRight, right);
            }

            if bottom == AUTO {
                YGNodeStyleSetMarginAuto(self.view_data().borrow().yg_node, YGEdgeBottom);
            } else {
                YGNodeStyleSetMargin(self.view_data().borrow().yg_node, YGEdgeBottom, bottom);
            }

            if left == AUTO {
                YGNodeStyleSetMarginAuto(self.view_data().borrow().yg_node, YGEdgeLeft);
            } else {
                YGNodeStyleSetMargin(self.view_data().borrow().yg_node, YGEdgeLeft, left);
            }
        }

        self.invalidate();
    }

    /**
     * Sets the top margin of the view, aka the space that separates
     * this view and the surrounding ones.
     *
     * Only works with views that have parents - top level views that are pushed
     * on the stack don't have parents.
     *
     * Use brls::View::AUTO to have the layout automatically select the
     * margin.
     */
    fn set_margin_top(&self, top: f32) {
        unsafe {
            if top == AUTO {
                YGNodeStyleSetMarginAuto(self.view_data().borrow().yg_node, YGEdgeTop);
            } else {
                YGNodeStyleSetMargin(self.view_data().borrow().yg_node, YGEdgeTop, top);
            }
        }

        // self.invalidate();
    }

    /**
     * Sets the right margin of the view, aka the space that separates
     * this view and the surrounding ones.
     *
     * Only works with views that have parents - top level views that are pushed
     * on the stack don't have parents.
     *
     * Use brls::View::AUTO to have the layout automatically select the
     * margin.
     */
    fn set_margin_right(&self, right: f32) {
        unsafe {
            if right == AUTO {
                YGNodeStyleSetMarginAuto(self.view_data().borrow().yg_node, YGEdgeRight);
            } else {
                YGNodeStyleSetMargin(self.view_data().borrow().yg_node, YGEdgeRight, right);
            }
        }

        self.invalidate();
    }

    fn margin_right(&self) -> f32 {
        ntz(unsafe { YGNodeStyleGetMargin(self.view_data().borrow().yg_node, YGEdgeRight).value })
    }
    fn margin_left(&self) -> f32 {
        ntz(unsafe { YGNodeStyleGetMargin(self.view_data().borrow().yg_node, YGEdgeLeft).value })
    }

    /**
     * Sets the bottom margin of the view, aka the space that separates
     * this view and the surrounding ones.
     *
     * Only works with views that have parents - top level views that are pushed
     * on the stack don't have parents.
     *
     * Use brls::View::AUTO to have the layout automatically select the
     * margin.
     */
    fn set_margin_bottom(&self, bottom: f32) {
        unsafe {
            if bottom == AUTO {
                YGNodeStyleSetMarginAuto(self.view_data().borrow().yg_node, YGEdgeBottom);
            } else {
                YGNodeStyleSetMargin(self.view_data().borrow().yg_node, YGEdgeBottom, bottom);
            }
        }

        self.invalidate();
    }

    /**
     * Sets the right margin of the view, aka the space that separates
     * this view and the surrounding ones.
     *
     * Only works with views that have parents - top level views that are pushed
     * on the stack don't have parents.
     *
     * Use brls::View::AUTO to have the layout automatically select the
     * margin.
     */
    fn set_margin_left(&self, left: f32) {
        unsafe {
            if left == AUTO {
                YGNodeStyleSetMarginAuto(self.view_data().borrow().yg_node, YGEdgeLeft);
            } else {
                YGNodeStyleSetMargin(self.view_data().borrow().yg_node, YGEdgeLeft, left);
            }
        }

        self.invalidate();
    }

    /**
     * Sets the visibility of the view.
     */
    fn set_visibility(&mut self, visibility: Visibility) {
        // Only change YG properties and invalidate if going from or to GONE
        if (self.view_data().borrow().visibility == Visibility::Gone && visibility != Visibility::Gone) || (self.view_data().borrow().visibility != Visibility::Gone && visibility == Visibility::Gone) {
            if visibility == Visibility::Gone {
                unsafe {
                    YGNodeStyleSetDisplay(self.view_data().borrow().yg_node, YGDisplayNone);
                }
            } else {
                unsafe {
                    YGNodeStyleSetDisplay(self.view_data().borrow().yg_node, YGDisplayFlex);
                }
            }

            self.invalidate();
        }

        self.view_data().borrow_mut().visibility = visibility;

        if visibility == Visibility::Visible {
            self.will_appear(false);
        } else {
            self.will_disappear(false);
        }
    }

    /**
     * Sets the top position of the view, in pixels.
     *
     * The behavior of this attribute changes depending on the
     * position type of the view.
     *
     * If relative, it will simply offset the view by the given amount.
     *
     * If absolute, it will behave like the "display: absolute;" CSS property
     * and move the view freely in its parent. Use 0 to snap to the parent top edge.
     * Absolute positioning ignores padding.
     *
     * Use View::AUTO to disable (not the same as 0).
     */
    fn set_position_top(&self, pos: f32) {
        unsafe {
            match pos == AUTO {
                true => {
                    YGNodeStyleSetPosition(self.view_data().borrow().yg_node, YGEdgeTop, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.view_data().borrow().yg_node, YGEdgeTop, pos),
            }
        }
        self.invalidate();
    }

    /**
     * Sets the right position of the view, in pixels.
     *
     * The behavior of this attribute changes depending on the
     * position type of the view.
     *
     * If relative, it will simply offset the view by the given amount.
     *
     * If absolute, it will behave like the "display: absolute;" CSS property
     * and move the view freely in its parent. Use 0 to snap to the parent right edge.
     * Absolute positioning ignores padding.
     *
     * Use View::AUTO to disable (not the same as 0).
     */
    fn set_position_right(&self, pos: f32) {
        unsafe {
            match pos == AUTO {
                true => {
                    YGNodeStyleSetPosition(self.view_data().borrow().yg_node, YGEdgeRight, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.view_data().borrow().yg_node, YGEdgeRight, pos),
            }
        }
        self.invalidate();
    }

    /**
     * Sets the bottom position of the view, in pixels.
     *
     * The behavior of this attribute changes depending on the
     * position type of the view.
     *
     * If relative, it will simply offset the view by the given amount.
     *
     * If absolute, it will behave like the "display: absolute;" CSS property
     * and move the view freely in its parent. Use 0 to snap to the parent bottom edge.
     * Absolute positioning ignores padding.
     *
     * Use View::AUTO to disable (not the same as 0).
     */
    fn set_position_bottom(&self, pos: f32) {
        unsafe {
            match pos == AUTO {
                true => {
                    YGNodeStyleSetPosition(self.view_data().borrow().yg_node, YGEdgeBottom, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.view_data().borrow().yg_node, YGEdgeBottom, pos),
            }
        }
        self.invalidate();
    }

    /**
     * Sets the left position of the view, in pixels.
     *
     * The behavior of this attribute changes depending on the
     * position type of the view.
     *
     * If relative, it will simply offset the view by the given amount.
     *
     * If absolute, it will behave like the "display: absolute;" CSS property
     * and move the view freely in its parent. Use 0 to snap to the parent left edge.
     * Absolute positioning ignores padding.
     *
     * Use View::AUTO to disable (not the same as 0).
     */
    fn set_position_left(&self, pos: f32) {
        unsafe {
            match pos == AUTO {
                true => {
                    YGNodeStyleSetPosition(self.view_data().borrow().yg_node, YGEdgeLeft, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.view_data().borrow().yg_node, YGEdgeLeft, pos),
            }
        }
        self.invalidate();
    }

    /**
     * Sets the top position of the view, in percents.
     *
     * The behavior of this attribute changes depending on the
     * position type of the view.
     */
    fn set_position_top_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetPositionPercent(self.view_data().borrow().yg_node, YGEdgeTop, percentage);
        }
        self.invalidate();
    }

    /**
     * Sets the right position of the view, in percents.
     *
     * The behavior of this attribute changes depending on the
     * position type of the view.
     */
    fn set_position_right_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetPositionPercent(self.view_data().borrow().yg_node, YGEdgeRight, percentage);
        }
        self.invalidate();
    }

    /**
     * Sets the bottom position of the view, in percentage.
     *
     * The behavior of this attribute changes depending on the
     * position type of the view.
     */
    fn set_position_bottom_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetPositionPercent(self.view_data().borrow().yg_node, YGEdgeBottom, percentage);
        }
        self.invalidate();
    }

    /**
     * Sets the left position of the view, in percents.
     *
     * The behavior of this attribute changes depending on the
     * position type of the view.
     */
    fn set_position_left_percentage(&self, percentage: f32) {
        unsafe {
            YGNodeStyleSetPositionPercent(self.view_data().borrow().yg_node, YGEdgeLeft, percentage);
        }
        self.invalidate();
    }

    /**
     * Sets the "position type" of the view, aka the behavior
     * of the 4 position attributes.
     *
     * Default is RELATIVE.
     */
    fn set_position_type(&self, _type: PositionType) {
        unsafe {
            match _type {
                PositionType::Relative => {
                    YGNodeStyleSetPositionType(self.view_data().borrow().yg_node, YGPositionTypeRelative)
                }
                PositionType::Absolute => {
                    YGNodeStyleSetPositionType(self.view_data().borrow().yg_node, YGPositionTypeAbsolute)
                }
            }
        }
        self.invalidate();
    }

    /**
     * Sets the id of the view.
     */
    fn set_id(&mut self, id: &str) {
        self.view_data().borrow_mut().id = id.into();
    }

    fn id(&self) -> String {
        self.view_data().borrow().id.clone()
    }

    /**
     * Overrides align items of the parent box.
     *
     * Default is AUTO.
     */
    fn set_align_self(&self, align_self: AlignSelf) {
        unsafe {
            match align_self {
                AlignSelf::Auto => YGNodeStyleSetAlignSelf(self.view_data().borrow().yg_node, YGAlignAuto),
                AlignSelf::FlexStart => {
                    YGNodeStyleSetAlignSelf(self.view_data().borrow().yg_node, YGAlignFlexStart)
                }
                AlignSelf::Center => YGNodeStyleSetAlignSelf(self.view_data().borrow().yg_node, YGAlignCenter),
                AlignSelf::FlexEnd => YGNodeStyleSetAlignSelf(self.view_data().borrow().yg_node, YGAlignFlexEnd),
                AlignSelf::Stretch => YGNodeStyleSetAlignSelf(self.view_data().borrow().yg_node, YGAlignStretch),
                AlignSelf::Baseline => {
                    YGNodeStyleSetAlignSelf(self.view_data().borrow().yg_node, YGAlignBaseline)
                }
                AlignSelf::SpaceBetween => {
                    YGNodeStyleSetAlignSelf(self.view_data().borrow().yg_node, YGAlignSpaceBetween)
                }
                AlignSelf::SpaceAround => {
                    YGNodeStyleSetAlignSelf(self.view_data().borrow().yg_node, YGAlignSpaceAround)
                }
            }
        }
        self.invalidate();
    }

    fn handle_xml_attributes(
        &mut self,
        element: roxmltree::Node,
        view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
    ) {
        panic!("Raw views cannot have child XML tags");
    }

    fn set_wireframe_enabled(&mut self, wireframe: bool) {
        self.view_data().borrow_mut().wireframe_enabled = wireframe;
    }

    fn is_wireframe_enabled(&self) -> bool {
        self.view_data().borrow().wireframe_enabled
    }

    /**
     * Called when the view will appear
     * on screen, before or after layout().
     *
     * Can be called if the view has
     * already appeared, so be careful.
     */
    fn will_appear(&self, reset_state: bool) {
        // Nothing to do
    }

    /**
     * Called when the view will disappear
     * from the screen.
     *
     * Can be called if the view has
     * already disappeared, so be careful.
     */
    fn will_disappear(&self, reset_state: bool) {
        // Nothing to do
    }
}
