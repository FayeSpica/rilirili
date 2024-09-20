use crate::core::application::ViewCreatorRegistry;
use crate::core::geometry::{Point, Rect, Size};
use crate::core::theme::{AUTO, nvg_rgb, nvg_rgba, theme, YG_UNDEFINED};
use crate::core::view_base::{AlignSelf, FocusDirection, PositionType, ShadowType, View, ViewBackground, ViewBase, Visibility};
use crate::core::view_style::ViewStyle;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
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
            self.parent().as_ref().unwrap().borrow().invalidate();
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
    fn set_visibility(&self, visibility: Visibility) {}

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

    fn apply_xml_attributes(
        &self,
        element: roxmltree::Node,
        view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
    ) {
        for attribute in element.attributes() {
            info!(
                "apply_xml_attributes: {} {}",
                attribute.name(),
                attribute.value()
            );
            self.apply_xml_attribute(attribute.name(), attribute.value());
        }
    }

    fn apply_xml_attribute(&self, name: &str, value: &str) -> bool {
        // String -> string
        if let Some(handler) = self.data().string_attributes.get(name) {
            if value.starts_with("@i18n/") {
                todo!();
                return true;
            }

            handler(value);
            return true;
        }

        // File path -> file path
        if value.starts_with("@res/") {
            let path = get_file_path_xml_attribute_value(value);

            if let Some(handler) = self.data().file_path_attributes.get(name) {
                handler(value);
                return true;
            } else {
                return false; // unknown res
            }
        } else {
            if let Some(handler) = self.data().file_path_attributes.get(name) {
                handler(value);
                return true;
            }

            // don't return false as it can be anything else
        }

        // Auto -> auto
        if "auto" == value {
            if let Some(handler) = self.data().auto_attributes.get(name) {
                handler();
                return true;
            } else {
                info!("{:?}", self.data().auto_attributes.keys());
                return false;
            }
        }

        // Ends with px -> float
        if value.ends_with("px") {
            // Strip the px and parse the float value
            let new_float = &value[..value.len() - 2];

            if let Ok(float_value) = f32::from_str(new_float) {
                if let Some(handler) = self.data().float_attributes.get(name) {
                    handler(float_value);
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Ends with % -> percentage
        if value.ends_with("%") {
            // Strip the % and parse the float value
            let new_float = &value[..value.len() - 1];

            if let Ok(float_value) = f32::from_str(new_float) {

                if float_value < -100.0 || float_value > 100.0 {
                    return false;
                }

                if let Some(handler) = self.data().float_attributes.get(name) {
                    handler(float_value);
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        // Starts with @style -> float
        else if value.starts_with("@style/") {
            // Parse the style name
            let style_name = &value[7..]; // length of "@style/"
            let float_value = style(style_name);

            if let Some(handler) = self.data().float_attributes.get(name) {
                handler(float_value);
                return true;
            } else {
                return false;
            }
        }
        // Starts with with # -> color
        else if value.starts_with("#") {
            // Parse the color
            // #RRGGBB format
            if value.len() == 7 {
                if let Some((r, g, b)) = hex_to_rgb(value) {
                    if let Some(handler) = self.data().color_attributes.get(name) {
                        handler(nvg_rgb(r, g, b));
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            // #RRGGBBAA format
            else if value.len() == 9 {
                if let Some((r, g, b, a)) = hex_to_rgba(value) {
                    if let Some(handler) = self.data().color_attributes.get(name) {
                        handler(nvg_rgba(r, g, b, a));
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        // Starts with @theme -> color
        else if value.starts_with("@theme/") {
            // Parse the color name
            let style_name = &value[7..]; // length of "@style/"
            let value = theme(style_name);

            if let Some(handler) = self.data().color_attributes.get(name) {
                handler(value);
                return true;
            } else {
                return false;
            }
        }
        // Equals true or false -> bool
        else if value == "true" || value == "false" {
            let bool_value = if value == "true" {
                true
            } else {
                false
            };

            if let Some(handler) = self.data().bool_attributes.get(name) {
                handler(bool_value);
                return true;
            } else {
                return false;
            }
        }

        // Valid float -> float, otherwise unknown attribute
        if let Ok(float_value) = f32::from_str(value) {
            if let Some(handler) = self.data().float_attributes.get(name) {
                handler(float_value);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn handle_xml_attributes(
        &mut self,
        element: roxmltree::Node,
        view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
    ) {
        panic!("Raw views cannot have child XML tags");
    }

    fn register_common_attributes(&mut self, view: Rc<RefCell<View>>) {
        // Width
        let view_clone = view.clone();
        self.register_auto_xml_attribute("width", Box::new(move || {
            // view_clone.borrow_mut().set_width(AUTO);
            let _ = view_clone.borrow().data();
            debug!("apply success");
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("width", Box::new(move |value| {
            view_clone.borrow_mut().set_width(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("width", Box::new(move |value| {
            view_clone.borrow_mut().set_width_percentage(value);
        }));

        // Height
        let view_clone = view.clone();
        self.register_auto_xml_attribute("height", Box::new(move || {
            view_clone.borrow_mut().set_height(AUTO);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("height", Box::new(move |value| {
            view_clone.borrow_mut().set_height(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("height", Box::new(move |value| {
            view_clone.borrow_mut().set_height_percentage(value);
        }));

        // Max width
        let view_clone = view.clone();
        self.register_auto_xml_attribute("maxWidth", Box::new(move || {
            view_clone.borrow_mut().set_max_width(AUTO);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("maxWidth", Box::new(move |value| {
            view_clone.borrow_mut().set_max_width(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("maxWidth", Box::new(move |value| {
            view_clone.borrow_mut().set_max_width_percentage(value);
        }));

        // Max Height
        let view_clone = view.clone();
        self.register_auto_xml_attribute("maxHeight", Box::new(move || {
            view_clone.borrow_mut().set_max_height(AUTO);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("maxHeight", Box::new(move |value| {
            view_clone.borrow_mut().set_max_height(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("maxHeight", Box::new(move |value| {
            view_clone.borrow_mut().set_max_height_percentage(value);
        }));

        // Grow and shrink
        let view_clone = view.clone();
        self.register_float_xml_attribute("grow", Box::new(move |value| {
            view_clone.borrow_mut().set_grow(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("shrink", Box::new(move |value| {
            view_clone.borrow_mut().set_shrink(value);
        }));

        // Alignment
        let view_clone = view.clone();
        self.register_string_xml_attribute("alignSelf", Box::new(move |value| {
            view_clone.borrow_mut().set_align_self( match value {
                "auto" => AlignSelf::Auto,
                "flexStart" => AlignSelf::FlexStart,
                "center" => AlignSelf::Center,
                "flexEnd" => AlignSelf::FlexEnd,
                "stretch" => AlignSelf::Stretch,
                "baseline" => AlignSelf::Baseline,
                "spaceBetween" => AlignSelf::SpaceBetween,
                "spaceAround" => AlignSelf::SpaceAround,
                &_ => AlignSelf::Auto,
            });
        }));

        // Margins top
        let view_clone = view.clone();
        self.register_float_xml_attribute("marginTop", Box::new(move |value| {
            view_clone.borrow_mut().set_margin_top(value);
        }));

        let view_clone = view.clone();
        self.register_auto_xml_attribute("marginTop", Box::new(move || {
            view_clone.borrow_mut().set_margin_top(AUTO);
        }));

        // Margins right
        let view_clone = view.clone();
        self.register_float_xml_attribute("marginRight", Box::new(move |value| {
            view_clone.borrow_mut().set_margin_right(value);
        }));

        let view_clone = view.clone();
        self.register_auto_xml_attribute("marginRight", Box::new(move || {
            view_clone.borrow_mut().set_margin_right(AUTO);
        }));

        // Margins bottom
        let view_clone = view.clone();
        self.register_float_xml_attribute("marginBottom", Box::new(move |value| {
            view_clone.borrow_mut().set_margin_bottom(value);
        }));

        let view_clone = view.clone();
        self.register_auto_xml_attribute("marginBottom", Box::new(move || {
            view_clone.borrow_mut().set_margin_bottom(AUTO);
        }));

        // Margins left
        let view_clone = view.clone();
        self.register_float_xml_attribute("marginLeft", Box::new(move |value| {
            view_clone.borrow_mut().set_margin_left(value);
        }));

        let view_clone = view.clone();
        self.register_auto_xml_attribute("marginLeft", Box::new(move || {
            view_clone.borrow_mut().set_margin_left(AUTO);
        }));

        // Line
        let view_clone = view.clone();
        self.register_color_xml_attribute("lineColor", Box::new(move |value| {
            view_clone.borrow_mut().set_line_color(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("lineTop", Box::new(move |value| {
            view_clone.borrow_mut().set_line_top(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("lineRight", Box::new(move |value| {
            view_clone.borrow_mut().set_line_right(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("lineBottom", Box::new(move |value| {
            view_clone.borrow_mut().set_line_bottom(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("lineLeft", Box::new(move |value| {
            view_clone.borrow_mut().set_line_left(value);
        }));

        // Position
        let view_clone = view.clone();
        self.register_float_xml_attribute("positionTop", Box::new(move |value| {
            view_clone.borrow_mut().set_position_top(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("positionRight", Box::new(move |value| {
            view_clone.borrow_mut().set_position_right(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("positionBottom", Box::new(move |value| {
            view_clone.borrow_mut().set_position_bottom(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("positionLeft", Box::new(move |value| {
            view_clone.borrow_mut().set_position_left(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("positionTop", Box::new(move |value| {
            view_clone.borrow_mut().set_position_top_percentage(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("positionRight", Box::new(move |value| {
            view_clone.borrow_mut().set_position_right_percentage(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("positionBottom", Box::new(move |value| {
            view_clone.borrow_mut().set_position_bottom_percentage(value);
        }));

        let view_clone = view.clone();
        self.register_percentage_xml_attribute("positionLeft", Box::new(move |value| {
            view_clone.borrow_mut().set_position_left_percentage(value);
        }));

        let view_clone = view.clone();
        self.register_string_xml_attribute("positionType", Box::new(move |value| {
            view_clone.borrow_mut().set_position_type( match value {
                "relative" => PositionType::Relative,
                "absolute" => PositionType::Absolute,
                &_ => PositionType::Relative,
            });
        }));

        // Custom focus routes
        let view_clone = view.clone();
        self.register_string_xml_attribute("focusUp", Box::new(move |value| {
            view_clone.borrow_mut().set_custom_navigation_route_by_id(FocusDirection::Up, value);
        }));

        let view_clone = view.clone();
        self.register_string_xml_attribute("focusRight", Box::new(move |value| {
            view_clone.borrow_mut().set_custom_navigation_route_by_id(FocusDirection::Right, value);
        }));

        let view_clone = view.clone();
        self.register_string_xml_attribute("focusDown", Box::new(move |value| {
            view_clone.borrow_mut().set_custom_navigation_route_by_id(FocusDirection::Down, value);
        }));

        let view_clone = view.clone();
        self.register_string_xml_attribute("focusLeft", Box::new(move |value| {
            view_clone.borrow_mut().set_custom_navigation_route_by_id(FocusDirection::Left, value);
        }));

        // Shape
        let view_clone = view.clone();
        self.register_color_xml_attribute("backgroundColor", Box::new(move |value| {
            view_clone.borrow_mut().set_background_color(value);
        }));

        let view_clone = view.clone();
        self.register_color_xml_attribute("borderColor", Box::new(move |value| {
            view_clone.borrow_mut().set_border_color(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("borderThickness", Box::new(move |value| {
            view_clone.borrow_mut().set_border_thickness(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("cornerRadius", Box::new(move |value| {
            view_clone.borrow_mut().set_corner_radius(value);
        }));

        let view_clone = view.clone();
        self.register_string_xml_attribute("shadowType", Box::new(move |value| {
            view_clone.borrow_mut().set_shadow_type( match value {
                "none" => ShadowType::None,
                "generic" => ShadowType::Generic,
                "custom" => ShadowType::Custom,
                &_ => ShadowType::None,
            });
        }));

        // Misc
        let view_clone = view.clone();
        self.register_string_xml_attribute("visibility", Box::new(move |value| {
            view_clone.borrow_mut().set_visibility( match value {
                "visible" => Visibility::Visible,
                "invisible" => Visibility::Invisible,
                "gone" => Visibility::Gone,
                &_ => Visibility::Visible,
            });
        }));

        let view_clone = view.clone();
        self.register_string_xml_attribute("id", Box::new(move |value| {
            view_clone.borrow_mut().set_id(value);
        }));

        let view_clone = view.clone();
        self.register_string_xml_attribute("background", Box::new(move |value| {
            view_clone.borrow_mut().set_background( match value {
                "sidebar" => ViewBackground::SideBar,
                "backdrop" => ViewBackground::BackDrop,
                &_ => ViewBackground::None,
            });
        }));

        let view_clone = view.clone();
        self.register_bool_xml_attribute("focusable", Box::new(move |value| {
            view_clone.borrow_mut().set_focusable(value);
        }));

        let view_clone = view.clone();
        self.register_bool_xml_attribute("wireframe", Box::new(move |value| {
            view_clone.borrow_mut().set_wireframe_enabled(value);
        }));

        // Highlight
        let view_clone = view.clone();
        self.register_bool_xml_attribute("hideHighlightBackground", Box::new(move |value| {
            view_clone.borrow_mut().set_hide_highlight_background(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("highlightPadding", Box::new(move |value| {
            view_clone.borrow_mut().set_highlight_padding(value);
        }));

        let view_clone = view.clone();
        self.register_float_xml_attribute("highlightCornerRadius", Box::new(move |value| {
            view_clone.borrow_mut().set_highlight_corner_radius(value);
        }));
    }

    fn register_auto_xml_attribute(&mut self, name: &str, handler: AutoAttributeHandler) {
        self.data_mut().auto_attributes.insert(name.into(), handler);
        self.data_mut().known_attributes.push(name.into());
    }

    fn register_float_xml_attribute(&mut self, name: &str, handler: FloatAttributeHandler) {
        self.data_mut().float_attributes.insert(name.into(), handler);
        self.data_mut().known_attributes.push(name.into());
    }

    fn register_percentage_xml_attribute(&mut self, name: &str, handler: FloatAttributeHandler) {
        self.data_mut().percentage_attributes.insert(name.into(), handler);
        self.data_mut().known_attributes.push(name.into());
    }

    fn register_string_xml_attribute(&mut self, name: &str, handler: StringAttributeHandler) {
        self.data_mut().string_attributes.insert(name.into(), handler);
        self.data_mut().known_attributes.push(name.into());
    }

    fn register_color_xml_attribute(&mut self, name: &str, handler: ColorAttributeHandler) {
        self.data_mut().color_attributes.insert(name.into(), handler);
        self.data_mut().known_attributes.push(name.into());
    }

    fn register_bool_xml_attribute(&mut self, name: &str, handler: BoolAttributeHandler) {
        self.data_mut().bool_attributes.insert(name.into(), handler);
        self.data_mut().known_attributes.push(name.into());
    }

    fn set_wireframe_enabled(&mut self, wireframe: bool) {
        self.data_mut().wireframe_enabled = wireframe;
    }

    fn is_wireframe_enabled(&self) -> bool {
        self.data().wireframe_enabled
    }
}
