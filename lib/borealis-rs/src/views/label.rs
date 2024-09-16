use crate::core::animation::Animatable;
use crate::core::theme::nvg_rgb;
use crate::core::time::Time;
use crate::core::view_base::{ViewBase, ViewData};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use nanovg_sys::NVGcolor;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum VerticalAlign {
    Baseline,
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CursorPosition {
    Unset = -2,
    End = -1,
    Start = 0,
}

/// Some text. The Label will automatically grow as much as possible.
/// If there is enough space, the label dimensions will fit the text.
/// If there is not enough horizontal space available, it will wrap and expand its height.
/// If there is not enough vertical space available to wrap, the text will be truncated instead.
/// The truncated text will animate if the Label or one of its parents is focused.
/// Animation will be disabled if the alignment is other than LEFT.
/// Warning: to wrap, the label width MUST be constrained
pub struct Label {
    pub view_data: ViewData,
    pub truncated_text: String,
    pub full_text: String,

    pub font: i32,
    pub font_size: f32,
    pub line_height: f32,
    pub font_quality: f32,

    pub text_color: NVGcolor,

    pub required_width: f32,
    pub ellipsis_width: f32,
    pub string_length: usize,

    pub single_line: bool,
    pub is_wrapping: bool,

    pub auto_animate: bool,
    pub animated: bool,
    pub animating: bool,

    pub cursor: i32,
    pub cursor_blink: Time,

    pub scrolling_animation: Animatable,
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
}

impl Label {
    pub fn new(id: &str) -> Self {
        let mut view_data = ViewData::default();
        view_data.id = id.into();
        Self {
            view_data,
            truncated_text: "".to_string(),
            full_text: "".to_string(),
            font: 0,
            font_size: 0.0,
            line_height: 0.0,
            font_quality: 0.0,
            text_color: nvg_rgb(0, 0, 0),
            required_width: 0.0,
            ellipsis_width: 0.0,
            string_length: 0,
            single_line: false,
            is_wrapping: false,
            auto_animate: false,
            animated: false,
            animating: false,
            cursor: 0,
            cursor_blink: 0,
            scrolling_animation: Animatable::new(0.0),
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Baseline,
        }
    }
}

pub trait LabelTrait: ViewTrait {
    fn label(&self) -> &Label;

    fn label_mut(&mut self) -> &mut Label;

    /**
     * Sets the text of the label.
     */
    fn set_text(&mut self, text: &str) {
        self.label_mut().truncated_text = text.to_string();
        self.label_mut().full_text = text.to_string();
        self.label_mut().string_length = text.len();

        self.invalidate();
    }

    /**
     * Sets the alignment of the text inside
     * the view. Will not move the view, only
     * the text inside.
     *
     * Default is CENTER.
     */
    fn set_horizontal_align(&mut self, align: HorizontalAlign) {
        self.label_mut().horizontal_align = align;
    }

    /**
     * Sets the alignment of the text inside
     * the view. Will not move the view, only
     * the text inside.
     *
     * Only applies to single-line labels.
     *
     * Default is CENTER.
     */
    fn set_vertical_align(&mut self, align: VerticalAlign) {
        self.label_mut().vertical_align = align;
    }

    fn set_font_size(&mut self, value: f32) {
        self.label_mut().font_size = value;

        self.invalidate();
    }

    fn set_font_quality(&mut self, value: f32) {
        self.label_mut().font_quality = value;

        self.invalidate();
    }

    fn set_line_height(&mut self, value: f32) {
        self.label_mut().line_height = value;

        self.invalidate();
    }

    fn set_text_color(&mut self, color: NVGcolor) {
        self.label_mut().text_color = color;
    }

    fn font(&self) -> i32 {
        self.label().font
    }

    fn font_size(&self) -> f32 {
        self.label().font_size
    }

    fn font_quality(&self) -> f32 {
        self.label().font_quality
    }

    fn line_height(&self) -> f32 {
        self.label().line_height
    }

    fn text_color(&self) -> NVGcolor {
        self.label().text_color
    }

    fn full_text(&self) -> &String {
        &self.label().full_text
    }

    fn set_required_width(&mut self, required_width: f32) {
        self.label_mut().required_width = required_width;
    }

    fn set_ellipsis_width(&mut self, ellipsis_width: f32) {
        self.label_mut().ellipsis_width = ellipsis_width;
    }
}

impl ViewTrait for Label {}

impl ViewDrawer for Label {}

impl ViewStyle for Label {}

impl ViewBase for Label {
    fn data(&self) -> &ViewData {
        &self.view_data
    }

    fn data_mut(&mut self) -> &mut ViewData {
        &mut self.view_data
    }
}

impl ViewLayout for Label {}

impl LabelTrait for Label {
    fn label(&self) -> &Label {
        self
    }

    fn label_mut(&mut self) -> &mut Label {
        self
    }
}
