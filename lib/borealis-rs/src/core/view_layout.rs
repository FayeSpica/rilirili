use yoga_sys::{YGDirection, YGEdge, YGNodeCalculateLayout, YGNodeLayoutGetHeight, YGNodeLayoutGetLeft, YGNodeLayoutGetTop, YGNodeLayoutGetWidth, YGNodeStyleSetAlignSelf, YGNodeStyleSetFlexGrow, YGNodeStyleSetFlexShrink, YGNodeStyleSetHeight, YGNodeStyleSetHeightAuto, YGNodeStyleSetMinHeight, YGNodeStyleSetMinHeightPercent, YGNodeStyleSetMinWidth, YGNodeStyleSetMinWidthPercent, YGNodeStyleSetPosition, YGNodeStyleSetPositionPercent, YGNodeStyleSetPositionType, YGNodeStyleSetWidth, YGNodeStyleSetWidthAuto};
use yoga_sys::YGPositionType::{YGPositionTypeAbsolute, YGPositionTypeRelative};
use yoga_sys::YGAlign::{YGAlignAuto, YGAlignBaseline, YGAlignCenter, YGAlignFlexEnd, YGAlignFlexStart, YGAlignSpaceAround, YGAlignSpaceBetween, YGAlignStretch};
use crate::core::geometry::{Point, Rect, Size};
use crate::core::theme::{AUTO, YG_UNDEFINED};
use crate::core::view_base::{AlignSelf, PositionType, View, ViewBase};

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
                true => {
                    YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeTop, YG_UNDEFINED)
                }
                false => YGNodeStyleSetPosition(self.data().yg_node, YGEdge::YGEdgeTop, pos),
            }
        }
        self.invalidate();
    }

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
}

impl ViewLayout for View {}