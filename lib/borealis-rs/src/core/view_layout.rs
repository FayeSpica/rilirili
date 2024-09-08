use std::cell::RefCell;
use std::rc::Rc;
use crate::core::geometry::{Point, Rect, Size};
use crate::core::theme::{AUTO, YG_UNDEFINED};
use crate::core::view_base::{AlignSelf, FocusDirection, PositionType, View, ViewBackground, ViewBase, Visibility};
use yoga_sys::YGAlign::{
    YGAlignAuto, YGAlignBaseline, YGAlignCenter, YGAlignFlexEnd, YGAlignFlexStart,
    YGAlignSpaceAround, YGAlignSpaceBetween, YGAlignStretch,
};
use yoga_sys::YGPositionType::{YGPositionTypeAbsolute, YGPositionTypeRelative};
use yoga_sys::{
    YGDirection, YGEdge, YGNodeCalculateLayout, YGNodeLayoutGetHeight, YGNodeLayoutGetLeft,
    YGNodeLayoutGetTop, YGNodeLayoutGetWidth, YGNodeStyleSetAlignSelf, YGNodeStyleSetFlexGrow,
    YGNodeStyleSetFlexShrink, YGNodeStyleSetHeight, YGNodeStyleSetHeightAuto,
    YGNodeStyleSetMinHeight, YGNodeStyleSetMinHeightPercent, YGNodeStyleSetMinWidth,
    YGNodeStyleSetMinWidthPercent, YGNodeStyleSetPosition, YGNodeStyleSetPositionPercent,
    YGNodeStyleSetPositionType, YGNodeStyleSetWidth, YGNodeStyleSetWidthAuto,
};
use crate::core::view_style::ViewStyle;

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
        return unsafe { YGNodeLayoutGetLeft(self.data().yg_node) };
    }

    fn local_y(&self) -> f32 {
        return unsafe { YGNodeLayoutGetTop(self.data().yg_node) };
    }

    fn width(&self) -> f32 {
        return unsafe { YGNodeLayoutGetWidth(self.data().yg_node) };
    }

    fn height(&self) -> f32 {
        return unsafe { YGNodeLayoutGetHeight(self.data().yg_node) };
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
            self.get_parent().invalidate();
        } else {
            unsafe {
                YGNodeCalculateLayout(
                    self.data().yg_node,
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
            YGNodeStyleSetMinWidthPercent(self.data().yg_node, 0.0);
            YGNodeStyleSetWidth(self.data().yg_node, width);
            YGNodeStyleSetMinWidth(self.data().yg_node, width);
        }
        self.invalidate();
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
            YGNodeStyleSetMinHeightPercent(self.data().yg_node, 0.0);
            YGNodeStyleSetHeight(self.data().yg_node, height);
            YGNodeStyleSetMinHeight(self.data().yg_node, height);
        }
        self.invalidate();
    }

    /**
     * Sets the preferred width and height of the view. Use brls::View::AUTO
     * to have the layout automatically resize the view.
     *
     * If set to anything else than AUTO, the view is guaranteed
     * to never shrink below the given height.
     */
    fn set_size(&self, size: Size) {
        todo!()
    }

    /**
     * Shortcut to setWidth + setHeight.
     *
     * Only does one layout pass instead of two when using the two methods separately.
     */
    fn set_dimensions(&self, width: f32, height: f32) {
        warn!("set_dimensions({}, {})", width, height);
        unsafe {
            YGNodeStyleSetMinWidthPercent(self.data().yg_node, 0.0);
            YGNodeStyleSetMinHeightPercent(self.data().yg_node, 0.0);

            match width == AUTO {
                true => {
                    YGNodeStyleSetWidthAuto(self.data().yg_node);
                    YGNodeStyleSetMinWidth(self.data().yg_node, YG_UNDEFINED);
                }
                false => {
                    YGNodeStyleSetWidth(self.data().yg_node, width);
                    YGNodeStyleSetMinWidth(self.data().yg_node, width);
                }
            }

            match height == AUTO {
                true => {
                    YGNodeStyleSetHeightAuto(self.data().yg_node);
                    YGNodeStyleSetMinHeight(self.data().yg_node, YG_UNDEFINED);
                }
                false => {
                    YGNodeStyleSetHeight(self.data().yg_node, height);
                    YGNodeStyleSetMinHeight(self.data().yg_node, height);
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
        todo!()
    }

    /**
     * Sets the preferred height of the view in percentage of
     * the parent view height. Between 0.0f and 100.0f.
     */
    fn set_height_percentage(&self, percentage: f32) {
        todo!()
    }

    /**
     * Sets the minimum width of the view, in pixels.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be less than the given min width.
     *
     * Use View::AUTO to disable the min width constraint.
     */
    fn set_min_width(&self, min_width: f32) {
        todo!()
    }

    /**
     * Sets the minimum height of the view, in pixels.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be less than the given max height.
     *
     * Use View::AUTO to disable the min height constraint.
     */
    fn set_min_height(&self, min_height: f32) {
        todo!()
    }

    /**
     * Sets the minimum width of the view, in parent width percentage.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be less than the given max width.
     *
     * Use View::AUTO to disable the min width constraint.
     */
    fn set_min_width_percentage(&self, percentage: f32) {
        todo!()
    }

    /**
     * Sets the minimum height of the view, in parent height percentage.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be less than the given max height.
     *
     * Use View::AUTO to disable the min height constraint.
     */
    fn set_min_height_percentage(&self, percentage: f32) {
        todo!()
    }

    /**
     * Sets the maximum width of the view, in pixels.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be larger than the given max width.
     *
     * Use View::AUTO to disable the max width constraint.
     */
    fn set_max_width(&self, min_width: f32) {
        todo!()
    }

    /**
     * Sets the maximum height of the view, in pixels.
     *
     * This constraint is stronger than the grow factor: the view
     * is guaranteed to never be larger than the given max height.
     *
     * Use View::AUTO to disable the max height constraint.
     */
    fn set_max_height(&self, min_height: f32) {
        todo!()
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
        todo!()
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
        todo!()
    }

    /**
     * Sets the grow factor of the view, aka the percentage
     * of remaining space to give this view, in the containing box axis.
     * Opposite of shrink.
     * Default is 0.0f;
     */
    fn set_grow(&self, grow: f32) {
        unsafe {
            YGNodeStyleSetFlexGrow(self.data().yg_node, grow);
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
            YGNodeStyleSetFlexShrink(self.data().yg_node, shrink);
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
        todo!()
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
        todo!()
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
        todo!()
    }

    fn margin_right(&self) -> f32 {
        todo!()
    }
    fn margin_left(&self) -> f32 {
        todo!()
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
    fn set_margin_bottom(&self, right: f32) {
        todo!()
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
        todo!()
    }

    /**
     * Sets the visibility of the view.
     */
    fn set_visibility(&self, visibility: Visibility) {

    }

    /**
     * Gets the visibility of the view.
     */
    fn visibility(&self) -> Visibility {
        todo!()
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
                    YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeTop, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeTop, pos),
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
                    YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeRight, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeRight, pos),
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
                    YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeBottom, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeBottom, pos),
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
                    YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeLeft, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeLeft, pos),
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
            YGNodeStyleSetPositionPercent(self.data().yg_node, YGEdge::YGEdgeTop, percentage);
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
            YGNodeStyleSetPositionPercent(self.data().yg_node, YGEdge::YGEdgeRight, percentage);
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
            YGNodeStyleSetPositionPercent(self.data().yg_node, YGEdge::YGEdgeBottom, percentage);
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
            YGNodeStyleSetPositionPercent(self.data().yg_node, YGEdge::YGEdgeLeft, percentage);
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
                    YGNodeStyleSetPositionType(self.data().yg_node, YGPositionTypeRelative)
                }
                PositionType::Absolute => {
                    YGNodeStyleSetPositionType(self.data().yg_node, YGPositionTypeAbsolute)
                }
            }
        }
        self.invalidate();
    }

    /**
     * Sets the id of the view.
     */
    fn set_id(&self, id: &str) {
        todo!()
    }

    /**
     * Overrides align items of the parent box.
     *
     * Default is AUTO.
     */
    fn set_align_self(&self, align_self: AlignSelf) {
        unsafe {
            match align_self {
                AlignSelf::Auto => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignAuto),
                AlignSelf::FlexStart => {
                    YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignFlexStart)
                }
                AlignSelf::Center => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignCenter),
                AlignSelf::FlexEnd => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignFlexEnd),
                AlignSelf::Stretch => YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignStretch),
                AlignSelf::Baseline => {
                    YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignBaseline)
                }
                AlignSelf::SpaceBetween => {
                    YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignSpaceBetween)
                }
                AlignSelf::SpaceAround => {
                    YGNodeStyleSetAlignSelf(self.data().yg_node, YGAlignSpaceAround)
                }
            }
        }
        self.invalidate();
    }
}
