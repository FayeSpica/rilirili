use nanovg_sys::NVGcolor;
use crate::core::view_base::{ShadowType, ViewBackground, ViewBase};

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
    fn set_line_color(&mut self, color: NVGcolor) {
        self.data_mut().line_color = color;
    }

    /**
     * Sets the top line thickness. Use setLineColor()
     * to change the line color.
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_top(&mut self, thickness: f32) {
        self.data_mut().line_top = thickness;
    }

    /**
     * Sets the right line thickness. Use setLineColor()
     * to change the line color.
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_right(&mut self, thickness: f32) {
        self.data_mut().line_right = thickness;
    }

    /**
     * Sets the bottom line thickness. Use setLineColor()
     * to change the line color.
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_bottom(&mut self, thickness: f32) {
        self.data_mut().line_bottom = thickness;
    }

    /**
     * Sets the left line thickness. Use setLineColor()
     * to change the line color.
     *
     * The "line" is separate from the shape "border".
     */
    fn set_line_left(&mut self, thickness: f32) {
        self.data_mut().line_left = thickness;
    }

    /**
     * Sets the view shape background color.
     */
    fn set_background_color(&mut self, color: NVGcolor)
    {
        self.data_mut().background_color = color;
        self.set_background(ViewBackground::ShapeColor);
    }

    /**
     * Sets the view shape border color.
     */
    fn set_border_color(&mut self, color: NVGcolor) {
        self.data_mut().border_color = color;
    }

    /**
     * Sets the view shape border thickness.
     */
    fn set_border_thickness(&mut self, thickness: f32) {
        self.data_mut().border_thickness = thickness;
    }

    fn border_thickness(&self) -> f32 {
        self.data().border_thickness
    }

    /**
     * Sets the view shape corner radius.
     * 0 means no rounded corners.
     */
    fn set_corner_radius(&mut self, radius: f32)
    {
        self.data_mut().corner_radius = radius;
    }

    fn corner_radius(&self) -> f32 {
        self.data().corner_radius
    }

    /**
     * Sets the view shape shadow type.
     * Default is NONE.
     */
    fn set_shadow_type(&mut self, _type: ShadowType) {
        self.data_mut().shadow_type = _type;
    }

    /**
     * Sets the shadow visibility.
     */
    fn set_shadow_visibility(&mut self, visible: bool)
    {
        self.data_mut().show_shadow = visible;
    }

    /**
     * If set to true, the highlight background will be hidden for this view
     * (the white rectangle that goes behind the view, replacing the usual background shape).
     */
    fn set_hide_highlight_background(&mut self, hide: bool)
    {
        self.data_mut().hide_highlight_background = hide;
    }

    /**
     * If set to true, the highlight border will be hidden for this view.
     */
    fn set_hide_highlight_border(&mut self, hide: bool)
    {
        self.data_mut().hide_highlight_border = hide;
    }

    /**
     * If set to true, the highlight will be hidden for this view.
     */
    fn set_hide_highlight(&mut self, hide: bool)
    {
        self.data_mut().hide_highlight = hide;
    }

    fn set_hide_click_animation(&mut self, hide: bool)
    {
        self.data_mut().hide_click_animation = hide;
    }

    /**
     * Sets the highlight padding of the view, aka the space between the
     * highlight rectangle and the view. The highlight rect is enlarged, the view is untouched.
     */
    fn set_highlight_padding(&mut self, padding: f32)
    {
        self.data_mut().highlight_padding = padding;
    }

    /**
     * Sets the highlight rectangle corner radius.
     */
    fn set_highlight_corner_radius(&mut self, radius: f32)
    {
        self.data_mut().highlight_corner_radius = radius;
    }
}