use std::cell::RefCell;
use std::cmp::min;
use std::ffi::{c_int, c_void, CString};
use std::rc::Rc;
use crate::core::animation::Animatable;
use crate::core::theme::{nvg_rgb, theme};
use crate::core::time::Time;
use crate::core::view_base::{PositionType, View, ViewBase, ViewData};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use nanovg_sys::{NVGcolor, nvgFillColor, nvgFontFaceId, nvgFontSize, nvgText, nvgTextAlign, nvgTextBounds, nvgTextBoxBounds, nvgTextLineHeight};
use yoga_sys::{YGMeasureMode, YGNodeGetContext, YGNodeRef, YGNodeSetContext, YGNodeSetMeasureFunc, YGNodeStyleSetMaxHeightPercent, YGNodeStyleSetMaxWidthPercent, YGSize};
use crate::core::application::FONT_REGULAR;
use crate::core::attribute::{register_bool_xml_attribute, register_color_xml_attribute, register_float_xml_attribute, register_string_xml_attribute};
use crate::core::font::font_stash;
use crate::core::frame_context::{frame_context, FrameContext};
use crate::core::style::style;
use crate::core::view_box::{BoxEnum, BoxTrait};
use crate::core::views::{add_view, get_view};
use crate::views::scrolling_frame::{BaseScrollingFrame, ScrollingFrame};

extern "C" fn label_measure_func(node: YGNodeRef, mut width: f32, mut widthMode: YGMeasureMode, height: f32, heightMode: YGMeasureMode) -> YGSize {
    debug!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!label_measure_func({}, {})", width, height);


    let mut size = YGSize{ width, height };

    // let retrieved_label_view = unsafe { YGNodeGetContext(node) as *mut Label };
    // let label = unsafe {
    //     retrieved_label_view.as_mut().unwrap()
    // };
    // let retrieved_view_ptr = unsafe { YGNodeGetContext(node) as *const RefCell<View> };
    // let retrieved_view = unsafe { Rc::from_raw(retrieved_view_ptr) };
    // let mut b = retrieved_view.try_borrow();

    let context_ptr = unsafe { YGNodeGetContext(node) };

    // 将 *mut c_void 转换回 Box<String> 并克隆内容
    let boxed_id = unsafe { &*(context_ptr as *mut String) };
    trace!("boxed_id: {}", boxed_id);
    let retrieved_view = get_view(boxed_id).unwrap();
    let mut b = retrieved_view.try_borrow().unwrap();

    let label = match & *b {
        View::Label(v) => v,
        _ => panic!()
    };

    // if label.label_data.borrow().full_text {
    //     return size;
    // }

    // XXX: workaround for a Yoga bug
    if widthMode == YGMeasureMode::YGMeasureModeAtMost && (width == 0.0 || width.is_nan()) {
        widthMode = YGMeasureMode::YGMeasureModeUndefined;
        width = f32::NAN;
    }

    let vg = frame_context();
    let mut ellipsis_width = 0f32;
    let mut required_width = 0f32;
    debug!("label.full_text: {:?}, label.font_size: {}, label.font: {}", label.label_data.borrow().full_text, label.label_data.borrow().font_size, label.label_data.borrow().font);
    unsafe {
        // Setup nvg state for the measurements
        nvgFontSize(vg, label.label_data.borrow().font_size);
        nvgTextAlign(vg, 1 | 8);
        nvgFontFaceId(vg, label.label_data.borrow().font);
        nvgTextLineHeight(vg, label.line_height());

        // // Measure the needed width for the ellipsis
        let mut bounds: [f32; 4] = [0.0, 0.0, 0.0, 0.0]; // width = xmax - xmin + some padding because nvgTextBounds isn't super precise
        let ellipsis_cstring = CString::new(ELLIPSIS).expect("CString::new failed");
        nvgTextBounds(vg, 0.0, 0.0, ellipsis_cstring.as_ptr(), std::ptr::null(), bounds.as_mut_ptr());
        ellipsis_width = bounds[2] - bounds[0] + 5.0;
        error!("ellipsis_width: {}", ellipsis_width);
        label.label_data.borrow_mut().ellipsis_width = ellipsis_width;

        // Measure the needed width for the fullText
        let mut bounds: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        nvgTextBounds(vg, 0.0, 0.0, label.label_data.borrow().full_text.as_c_str().as_ptr(), std::ptr::null(), bounds.as_mut_ptr());
        required_width = bounds[2] - bounds[0] + 5.0;
        error!("required_width: {}", required_width);
        label.label_data.borrow_mut().required_width = required_width;
    }

    // XXX: This is an approximation since the given width here may not match the actual final width of the view
    let available_width = if width.is_nan() {
        f32::MAX
    } else {
        width
    };

    // Width
    if widthMode == YGMeasureMode::YGMeasureModeUndefined || widthMode == YGMeasureMode::YGMeasureModeAtMost {
        // Grow the label horizontally as much as possible
        if widthMode == YGMeasureMode::YGMeasureModeAtMost {
            size.width = f32::min(required_width, available_width);
        } else {
            size.width = required_width;
        }
    } else if widthMode == YGMeasureMode::YGMeasureModeExactly {
        size.width = width;
    } else {
        panic!("Unsupported Label width measure mode: ");
    }

    // Height
    // Measure the required height, with wrapping

    // Is wrapping necessary and allowed ?
    if available_width < required_width && !label.label_data.borrow().single_line {
        unsafe {
            let mut box_bounds: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            nvgTextBoxBounds(vg, 0.0, 0.0, available_width, label.label_data.borrow().full_text.as_c_str().as_ptr(), std::ptr::null(), box_bounds.as_mut_ptr());

            let required_height = box_bounds[3] - box_bounds[1];

            // Undefined height mode, always wrap
            if heightMode == YGMeasureMode::YGMeasureModeUndefined {
                // label.is_wrapping = true;
                size.height = required_height;
            }
            // At most height mode, see if we have enough space
            else if heightMode == YGMeasureMode::YGMeasureModeAtMost {
                todo!()
            }
            else if heightMode == YGMeasureMode::YGMeasureModeExactly {
                todo!()
            }
            else {
                panic!("Unsupported Label height measure mode: ")
            }

        }
    }
    // No wrapping necessary or allowed, return the normal height
    else {

    }
    size.width = 100.0;
    size.height = 10.0;
    size
}

const ELLIPSIS: &str = "\u{2026}";

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

pub struct LabelData {
    pub truncated_text: CString,
    pub full_text: CString,

    pub font: c_int,
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

/// Some text. The Label will automatically grow as much as possible.
/// If there is enough space, the label dimensions will fit the text.
/// If there is not enough horizontal space available, it will wrap and expand its height.
/// If there is not enough vertical space available to wrap, the text will be truncated instead.
/// The truncated text will animate if the Label or one of its parents is focused.
/// Animation will be disabled if the alignment is other than LEFT.
/// Warning: to wrap, the label width MUST be constrained
pub struct Label {
    pub label_data: Rc<RefCell<LabelData>>,
    pub view_data: Rc<RefCell<ViewData>>
}

impl Default for LabelData {
    fn default() -> Self {
        let view_data = ViewData::default();
        let mut s = Self {
            truncated_text: CString::new("truncated_text").unwrap(),
            full_text: CString::new("full_text").unwrap(),
            font: font_stash(FONT_REGULAR).unwrap(),
            font_size: style("brls/label/default_font_size"),
            line_height: style("brls/label/default_line_height"),
            font_quality: 0.0,
            text_color: theme("brls/text"),
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
        };
        s
    }
}

impl Default for Label {
    fn default() -> Self {
        let mut s = Self {
            label_data: Default::default(),
            view_data: Default::default(),
        };

        s.set_highlight_padding(style("brls/label/highlight_padding"));

        unsafe {
            // Setup the custom measure function
            YGNodeSetMeasureFunc(s.view_data().borrow().yg_node, Some(label_measure_func));

            // Set the max width and height to 100% to avoid overflowing
            // The view will be shortened if the text is too long
            YGNodeStyleSetMaxWidthPercent(s.view_data().borrow().yg_node, 100.0);
            YGNodeStyleSetMaxHeightPercent(s.view_data().borrow().yg_node, 100.0);
        }

        // Register XML attributes
        register_string_xml_attribute("text", Box::new(|view, value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_text(value),
                _ => {}
            }
        }));

        register_float_xml_attribute("fontSize", Box::new(|view, value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_font_size(value),
                _ => {}
            }
        }));

        register_color_xml_attribute("textColor", Box::new(|view, value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_text_color(value),
                _ => {}
            }
        }));

        register_float_xml_attribute("lineHeight", Box::new(|view, value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_line_height(value),
                _ => {}
            }
        }));

        register_bool_xml_attribute("animated", Box::new(|view, value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_animated(value),
                _ => {}
            }
        }));

        register_bool_xml_attribute("autoAnimate", Box::new(|view, value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_auto_animate(value),
                _ => {}
            }
        }));

        register_bool_xml_attribute("singleLine", Box::new(|view, value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_single_line(value),
                _ => {}
            }
        }));

        register_string_xml_attribute("horizontalAlign", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_horizontal_align(match value {
                    "left" => HorizontalAlign::Left,
                    "center" => HorizontalAlign::Center,
                    "right" => HorizontalAlign::Right,
                    &_ => HorizontalAlign::Center,
                }),
                _ => {}
            }
        }));

        register_string_xml_attribute("verticalAlign", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Label(v) => v.set_vertical_align(match value {
                    "baseline" => VerticalAlign::Baseline,
                    "top" => VerticalAlign::Top,
                    "center" => VerticalAlign::Center,
                    "bottom" => VerticalAlign::Bottom,
                    &_ => VerticalAlign::Center,
                }),
                _ => {}
            }
        }));

        s
    }
}

impl Label {
    pub fn create() -> Rc<RefCell<View>> {
        let l = Label::default();
        let v = Rc::new(RefCell::new(View::Label(l)));
        add_view(&v.borrow().id(), v.clone());
        let boxed_id = Box::new(String::from(v.borrow().id()));
        unsafe {
            YGNodeSetContext(v.borrow().view_data().borrow().yg_node, Box::into_raw(boxed_id) as *mut c_void);
        }
        v
    }
}

pub trait LabelTrait: ViewTrait {

    fn label_data(&self) -> &Rc<RefCell<LabelData>>;

    /**
     * Sets the text of the label.
     */
    fn set_text(&mut self, text: &str) {
        self.label_data().borrow_mut().truncated_text = CString::new(text).unwrap();
        self.label_data().borrow_mut().full_text = CString::new(text).unwrap();
        self.label_data().borrow_mut().string_length = text.len();

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
        self.label_data().borrow_mut().horizontal_align = align;
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
        self.label_data().borrow_mut().vertical_align = align;
    }

    fn set_font_size(&mut self, value: f32) {
        self.label_data().borrow_mut().font_size = value;

        self.invalidate();
    }

    fn set_font_quality(&mut self, value: f32) {
        self.label_data().borrow_mut().font_quality = value;

        self.invalidate();
    }

    fn set_line_height(&mut self, value: f32) {
        self.label_data().borrow_mut().line_height = value;

        self.invalidate();
    }

    fn set_text_color(&mut self, color: NVGcolor) {
        self.label_data().borrow_mut().text_color = color;
    }

    fn font(&self) -> c_int {
        self.label_data().borrow().font
    }

    fn font_size(&self) -> f32 {
        self.label_data().borrow().font_size
    }

    fn font_quality(&self) -> f32 {
        self.label_data().borrow().font_quality
    }

    fn line_height(&self) -> f32 {
        self.label_data().borrow().line_height
    }

    fn text_color(&self) -> NVGcolor {
        self.label_data().borrow().text_color
    }

    fn full_text(&self) -> CString {
        self.label_data().borrow().full_text.clone()
    }

    fn set_required_width(&mut self, required_width: f32) {
        self.label_data().borrow_mut().required_width = required_width;
    }

    fn set_ellipsis_width(&mut self, ellipsis_width: f32) {
        self.label_data().borrow_mut().ellipsis_width = ellipsis_width;
    }

    fn set_animated(&mut self, animated: bool) {
        if animated == self.label_data().borrow().animated || self.label_data().borrow().is_wrapping || self.label_data().borrow().horizontal_align != HorizontalAlign::Left {
            return;
        }

        self.label_data().borrow_mut().animated = animated;

        self.reset_scrolling_animation();
    }

    fn set_auto_animate(&mut self, value: bool) {

    }

    fn set_single_line(&mut self, value: bool) {

    }

    fn reset_scrolling_animation(&mut self) {

    }

    fn draw(&mut self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        trace!("Label::draw ({},{},{},{}), font: {}, font_size: {}, text: {:?}", x, y, width, height, self.font(), self.font_size(), self.label_data().borrow().truncated_text);
        if width == 0.0 {
            warn!("zero width label");
            return;
        }

        unsafe {
            nvgFontSize(ctx.context, self.font_size());
            nvgFontFaceId(ctx.context, self.font());
            nvgTextLineHeight(ctx.context, self.line_height());
            nvgFillColor(ctx.context, self.a(self.text_color()));
        }

        // Animated text
        if self.label_data().borrow().animating {
            trace!("a");
        }
        // Wrapped text
        else if self.label_data().borrow().is_wrapping {
            trace!("b");
        }
        // Truncated text
        else {
            trace!("c");

            let mut text_x = x;
            let mut text_y = y;

            if self.label_data().borrow().horizontal_align == HorizontalAlign::Center {
                text_x += width / 2.0;
            }
            else if self.label_data().borrow().horizontal_align == HorizontalAlign::Right {
                text_x += width;
            }

            if self.label_data().borrow().vertical_align == VerticalAlign::Center || self.label_data().borrow().vertical_align == VerticalAlign::Baseline {
                text_y += height / 2.0;
            }
            else if self.label_data().borrow().vertical_align == VerticalAlign::Bottom {
                text_y += height;
            }

            unsafe {
                nvgText(ctx.context, text_x, text_y, self.label_data().borrow().truncated_text.as_ptr(), std::ptr::null());
            }
        }
    }
}

impl ViewTrait for Label {}

impl ViewDrawer for Label {
    fn draw(&mut self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        LabelTrait::draw(self, ctx, x, y, width, height);
    }
}

impl ViewStyle for Label {}

impl ViewBase for Label {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        &self.view_data
    }
}

impl ViewLayout for Label {}

impl LabelTrait for Label {
    fn label_data(&self) -> &Rc<RefCell<LabelData>> {
        &self.label_data
    }
}
