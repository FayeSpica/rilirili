use crate::core::bind::BoundView;
use crate::core::style::style;
use crate::core::theme::{nvg_rgb, theme};
use crate::core::view_base::{ShadowType, View, ViewBackground, ViewBase, ViewData};
use crate::core::view_box::{BoxEnum, BoxTrait, BoxViewData};
use crate::core::view_drawer::ViewDrawer;
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::label::{Label, LabelTrait};
use nanovg_sys::NVGcolor;
use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;

/// Style and colors of different buttons styles
/// Border color entries can be empty if thickness is 0
/// Highlight padding can be empty
/// Background color entries can be empty
/// Border thickness can be empty
/// The rest is mandatory
pub struct ButtonStyle {
    shadow_type: ShadowType,
    hide_highlight_background: bool,

    /// Style entries
    highlight_padding: String,
    border_thickness: String,

    /// Theme entries
    enabled_background_color: String,
    enabled_label_color: String,
    enabled_border_color: String,

    disabled_background_color: String,
    disabled_label_color: String,
    disabled_border_color: String,
}

/// primary action button (different background color than default, to catch the eye)
pub fn button_style_primary() -> ButtonStyle {
    ButtonStyle {
        shadow_type: ShadowType::Generic,
        hide_highlight_background: true,
        highlight_padding: "brls/button/primary_highlight_padding".to_string(),
        border_thickness: "".to_string(),
        enabled_background_color: "brls/button/primary_enabled_background".to_string(),
        enabled_label_color: "brls/button/primary_enabled_text".to_string(),
        enabled_border_color: "".to_string(),
        disabled_background_color: "brls/button/primary_disabled_background".to_string(),
        disabled_label_color: "brls/button/primary_disabled_text".to_string(),
        disabled_border_color: "".to_string(),
    }
}

/// between primary and default - text color is different, background color is the same
pub fn button_style_highlight() -> ButtonStyle {
    ButtonStyle {
        shadow_type: ShadowType::Generic,
        hide_highlight_background: true,
        highlight_padding: "".to_string(),
        border_thickness: "".to_string(),
        enabled_background_color: "brls/button/default_enabled_background".to_string(),
        enabled_label_color: "brls/button/highlight_enabled_text".to_string(),
        enabled_border_color: "".to_string(),
        disabled_background_color: "brls/button/default_disabled_background".to_string(),
        disabled_label_color: "brls/button/highlight_disabled_text".to_string(),
        disabled_border_color: "".to_string(),
    }
}

/// default, plain button
pub fn button_style_default() -> ButtonStyle {
    ButtonStyle {
        shadow_type: ShadowType::Generic,
        hide_highlight_background: true,
        highlight_padding: "".to_string(),
        border_thickness: "".to_string(),
        enabled_background_color: "brls/button/default_enabled_background".to_string(),
        enabled_label_color: "brls/button/default_enabled_text".to_string(),
        enabled_border_color: "".to_string(),
        disabled_background_color: "brls/button/default_disabled_background".to_string(),
        disabled_label_color: "brls/button/default_disabled_text".to_string(),
        disabled_border_color: "".to_string(),
    }
}

/// text and a border
pub fn button_style_bordered() -> ButtonStyle {
    ButtonStyle {
        shadow_type: ShadowType::None,
        hide_highlight_background: false,
        highlight_padding: "".to_string(),
        border_thickness: "brls/button/border_thickness".to_string(),
        enabled_background_color: "".to_string(),
        enabled_label_color: "brls/button/default_enabled_text".to_string(),
        enabled_border_color: "brls/button/enabled_border_color".to_string(),
        disabled_background_color: "".to_string(),
        disabled_label_color: "brls/button/default_disabled_text".to_string(),
        disabled_border_color: "brls/button/disabled_border_color".to_string(),
    }
}

/// only text
pub fn button_style_borderless() -> ButtonStyle {
    ButtonStyle {
        shadow_type: ShadowType::None,
        hide_highlight_background: false,
        highlight_padding: "".to_string(),
        border_thickness: "".to_string(),
        enabled_background_color: "".to_string(),
        enabled_label_color: "brls/button/default_enabled_text".to_string(),
        enabled_border_color: "".to_string(),
        disabled_background_color: "".to_string(),
        disabled_label_color: "brls/button/default_disabled_text".to_string(),
        disabled_border_color: "".to_string(),
    }
}

pub enum ButtonState {
    Enabled = 0, // the user can select and click on the button
    Disabled,    // the user can select but not click on the button (greyed out)
}

/// A button
pub struct Button {
    view_data: ViewData,
    style: ButtonStyle,
    state: ButtonState,

    text_color: NVGcolor,
    text_color_overwritten: bool,
    label: Label,
}

impl Button {
    pub fn new() -> Self {
        Self {
            view_data: Default::default(),
            style: button_style_default(),
            state: ButtonState::Enabled,
            text_color: nvg_rgb(0, 0, 0),
            text_color_overwritten: false,
            label: Label::default(),
        }
    }
}

pub trait ButtonTrait: BoxTrait {
    fn this(&self) -> &Button;

    fn this_mut(&mut self) -> &mut Button;

    fn apply_style(&mut self) {
        self.set_shadow_type(self.this().style.shadow_type);
        self.set_hide_highlight_background(self.this().style.hide_highlight_background);

        if !self.this().style.highlight_padding.is_empty() {
            self.set_highlight_padding(style(&self.this().style.highlight_padding));
        } else {
            self.set_highlight_padding(0.0);
        }

        if !self.this().style.border_thickness.is_empty() {
            self.set_border_thickness(style(&self.this().style.border_thickness));
        } else {
            self.set_border_thickness(0.0);
        }

        let (background_color, text_color, border_color) = match self.this().state {
            ButtonState::Enabled => (
                self.this().style.enabled_background_color.clone(),
                self.this().style.enabled_label_color.clone(),
                self.this().style.enabled_border_color.clone(),
            ),
            ButtonState::Disabled => (
                self.this().style.disabled_background_color.clone(),
                self.this().style.disabled_label_color.clone(),
                self.this().style.disabled_border_color.clone(),
            ),
        };

        if !background_color.is_empty() {
            self.set_background_color(theme(&background_color));
        } else {
            self.set_background(ViewBackground::None);
        }

        if self.this().text_color_overwritten {
            let color = self.this().text_color.clone();
            self.this_mut().label.set_text_color(color);
        } else {
            self.this_mut().label.set_text_color(theme(&text_color));
        }

        if self.this().border_thickness() > 0.0 {
            self.this_mut().set_border_color(theme(&border_color));
        }
    }

    fn on_focus_gained(&mut self) {
        self.view_data().borrow_mut().focused = true;
        self.set_shadow_visibility(false);
    }
    fn on_focus_lost(&mut self) {
        self.view_data().borrow_mut().focused = false;
        self.set_shadow_visibility(true);
    }

    /**
     * Sets the style of the button. can be a pointer to one of the
     * BUTTONSTYLE constants or any other user created style.
     */
    fn set_style(&mut self, style: ButtonStyle) {
        self.this_mut().style = style;
        self.this_mut().text_color_overwritten = false;
        self.apply_style();
    }

    /**
     * Sets the state of the button.
     */
    fn set_state(&mut self, state: ButtonState) {
        self.this_mut().state = state;
        self.apply_style();
    }

    /**
     * Sets the text of the button.
     */
    fn set_text(&mut self, text: &str) {
        self.this_mut().label.set_text(text);
    }

    /**
     * Sets the font sise of the button.
     */
    fn set_font_size(&mut self, value: f32) {
        self.this_mut().label.set_font_size(value);
    }

    /**
     * Override style text color of the button.
     */
    fn set_text_color(&mut self, color: NVGcolor) {
        self.this_mut().text_color = color;
        self.this_mut().text_color_overwritten = true;
        self.apply_style();
    }

    /**
     * Returns the text of the button
     */
    fn text(&self) -> CString {
        self.this().label.full_text()
    }
}

impl BoxTrait for Button {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        todo!()
    }
}

impl ViewDrawer for Button {}

impl ViewLayout for Button {}

impl ViewStyle for Button {}

impl ViewBase for Button {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        todo!()
    }
}

impl ButtonTrait for Button {
    fn this(&self) -> &Button {
        self
    }

    fn this_mut(&mut self) -> &mut Button {
        self
    }
}
