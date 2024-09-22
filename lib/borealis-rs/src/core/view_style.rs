use std::cell::RefCell;
use std::rc::Rc;
use crate::core::view_base::{FocusDirection, ShadowType, View, ViewBackground, ViewBase};
use nanovg_sys::NVGcolor;
use crate::core::view_box::Direction;

/// -----------------------------------------------------------
/// Styling and view shape properties
/// -----------------------------------------------------------
pub trait ViewStyle: ViewBase {
    /**
     * Sets the line color for the view. To be used with setLineTop(),
     * setLineRight()...
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_color(&self, color: NVGcolor) {
        self.view_data().borrow_mut().line_color = color;
    }

    /**
     * Sets the top line thickness. Use setLineColor()
     * to change the line color.
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_top(&self, thickness: f32) {
        self.view_data().borrow_mut().line_top = thickness;
    }

    /**
     * Sets the right line thickness. Use setLineColor()
     * to change the line color.
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_right(&self, thickness: f32) {
        self.view_data().borrow_mut().line_right = thickness;
    }

    /**
     * Sets the bottom line thickness. Use setLineColor()
     * to change the line color.
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_bottom(&self, thickness: f32) {
        self.view_data().borrow_mut().line_bottom = thickness;
    }

    /**
     * Sets the left line thickness. Use setLineColor()
     * to change the line color.
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_left(&self, thickness: f32) {
        self.view_data().borrow_mut().line_left = thickness;
    }

    /**
     * Sets the view shape background color.
     */
    fn set_background_color(&self, color: NVGcolor) {
        self.view_data().borrow_mut().background_color = color;
        self.set_background(ViewBackground::ShapeColor);
    }

    /**
     * Sets the view shape border color.
     */
    fn set_border_color(&self, color: NVGcolor) {
        self.view_data().borrow_mut().border_color = color;
    }

    /**
     * Sets the view shape border thickness.
     */
    fn set_border_thickness(&self, thickness: f32) {
        self.view_data().borrow_mut().border_thickness = thickness;
    }

    fn border_thickness(&self) -> f32 {
        self.view_data().borrow().border_thickness
    }

    /**
     * Sets the view shape corner radius.
     * 0 means no rounded corners.
     */
    fn set_corner_radius(&self, radius: f32) {
        self.view_data().borrow_mut().corner_radius = radius;
    }

    fn corner_radius(&self) -> f32 {
        self.view_data().borrow().corner_radius
    }

    /**
     * Sets the view shape shadow type.
     * Default is NONE.
     */
    fn set_shadow_type(&self, _type: ShadowType) {
        self.view_data().borrow_mut().shadow_type = _type;
    }

    /**
     * Sets the shadow visibility.
     */
    fn set_shadow_visibility(&self, visible: bool) {
        self.view_data().borrow_mut().show_shadow = visible;
    }

    /**
     * If set to true, the highlight background will be hidden for this view
     * (the white rectangle that goes behind the view, replacing the usual background shape).
     */
    fn set_hide_highlight_background(&self, hide: bool) {
        self.view_data().borrow_mut().hide_highlight_background = hide;
    }

    /**
     * If set to true, the highlight border will be hidden for this view.
     */
    fn set_hide_highlight_border(&self, hide: bool) {
        self.view_data().borrow_mut().hide_highlight_border = hide;
    }

    /**
     * If set to true, the highlight will be hidden for this view.
     */
    fn set_hide_highlight(&self, hide: bool) {
        self.view_data().borrow_mut().hide_highlight = hide;
    }

    fn set_hide_click_animation(&self, hide: bool) {
        self.view_data().borrow_mut().hide_click_animation = hide;
    }

    /**
     * Sets the highlight padding of the view, aka the space between the
     * highlight rectangle and the view. The highlight rect is enlarged, the view is untouched.
     */
    fn set_highlight_padding(&self, padding: f32) {
        self.view_data().borrow_mut().highlight_padding = padding;
    }

    /**
     * Sets the highlight rectangle corner radius.
     */
    fn set_highlight_corner_radius(&self, radius: f32) {
        self.view_data().borrow_mut().highlight_corner_radius = radius;
    }

    /**
     * Sets a custom navigation route from this view to the target one.
     */
    fn set_custom_navigation_route_by_ptr(&self, direction: FocusDirection, target: Rc<RefCell<View>>) {
        if !self.is_focusable() {
            panic!("Only focusable views can have a custom navigation route")
        }

        self.view_data().borrow_mut().custom_focus_by_ptr.insert(direction, target);
    }

    /**
     * Sets a custom navigation route from this view to the target one, by ID.
     * The final target view will be the "nearest" with the given ID.
     *
     * Resolution of the ID to View is made when the navigation event occurs, not when the
     * route is registered.
     */
    fn set_custom_navigation_route_by_id(&self, direction: FocusDirection, target_id: &str) {
        if !self.is_focusable() {
            panic!("Only focusable views can have a custom navigation route")
        }

        self.view_data().borrow_mut().custom_focus_by_id.insert(direction, String::from(target_id));
    }

    fn has_custom_navigation_route_by_ptr(&self, direction: FocusDirection) -> bool {
        !self.view_data().borrow().custom_focus_by_ptr.is_empty()
    }

    fn has_custom_navigation_route_by_id(&self, direction: FocusDirection) -> bool {
        !self.view_data().borrow().custom_focus_by_id.is_empty()
    }

    fn custom_navigation_route_by_ptr(&self, direction: FocusDirection) -> Option<Rc<RefCell<View>>> {
        self.view_data().borrow().custom_focus_by_ptr.get(&direction).cloned()
    }

    fn custom_navigation_route_by_id(&self, direction: FocusDirection) -> Option<String> {
        self.view_data().borrow().custom_focus_by_id.get(&direction).cloned()
    }
}
