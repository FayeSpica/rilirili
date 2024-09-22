use crate::core::application::ViewCreatorRegistry;
use crate::core::frame_context::FrameContext;
use crate::core::view_base::{View, ViewBase, ViewData};
use crate::core::view_creator::create_from_xml_element;
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::views::applet_frame::AppletFrame;
use crate::views::bottom_bar::BottomBar;
use crate::views::button::Button;
use crate::views::debug_layer::DebugLayer;
use crate::views::dialog::Dialog;
use crate::views::dropdown::Dropdown;
use crate::views::edit_text_dialog::EditTextDialog;
use crate::views::h_scrolling_frame::HScrollingFrame;
use crate::views::header::Header;
use crate::views::hint::{Hint, Hints};
use crate::views::image::Image;
use crate::views::recycler::{RecyclerCell, RecyclerHeader};
use crate::views::scrolling_frame::{BaseScrollingFrame, ScrollingFrame};
use crate::views::slider::Slider;
use crate::views::tab_frame::TabFrame;
#[cfg(feature = "mpv")]
use crate::views::video::{Video, VideoTrait};
use roxmltree::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yoga_sys::YGEdge::{YGEdgeBottom, YGEdgeLeft, YGEdgeRight, YGEdgeTop};
use yoga_sys::{YGAlign, YGDirection, YGFlexDirection, YGJustify, YGNodeGetChildCount, YGNodeInsertChild, YGNodeRemoveChild, YGNodeStyleGetPadding, YGNodeStyleSetAlignItems, YGNodeStyleSetDirection, YGNodeStyleSetFlexDirection, YGNodeStyleSetJustifyContent, YGNodeStyleSetPadding};
use crate::core::attribute::{register_float_xml_attribute, register_string_xml_attribute};
use crate::views::label::{HorizontalAlign, LabelTrait};

pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    // SpaceEvenly,
}

pub enum AlignItems {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}

#[derive(Copy, Clone)]
pub enum Axis {
    Row,
    Column,
}

pub enum Direction {
    Inherit,
    LeftToRight,
    RightToLeft,
}

pub struct BoxView {
    pub box_view_data: Rc<RefCell<BoxViewData>>,
    pub view_data: Rc<RefCell<ViewData>>,
}

impl Default for BoxView {
    fn default() -> Self {
        let mut s = Self {
            box_view_data: Default::default(),
            view_data: Default::default(),
        };

        s.set_axis(Axis::Row);

        register_string_xml_attribute("axis", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_axis(match value {
                    "row" => Axis::Row,
                    "column" => Axis::Column,
                    &_ => Axis::Row,
                }),
                _ => {}
            }
        }));

        register_string_xml_attribute("direction", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_direction(match value {
                    "inherit" => Direction::Inherit,
                    "leftToRight" => Direction::LeftToRight,
                    "rightToLeft" => Direction::RightToLeft,
                    &_ => Direction::LeftToRight,
                }),
                _ => {}
            }
        }));

        register_string_xml_attribute("justifyContent", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_justify_content(match value {
                    "flexStart" => JustifyContent::FlexStart,
                    "center" => JustifyContent::Center,
                    "flexEnd" => JustifyContent::FlexEnd,
                    "spaceBetween" => JustifyContent::SpaceBetween,
                    "spaceAround" => JustifyContent::SpaceAround,
                    &_ => JustifyContent::FlexStart
                }),
                _ => {}
            }
        }));

        register_string_xml_attribute("alignItems", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_align_items(match value {
                    "auto" => AlignItems::Auto,
                    "flexStart" => AlignItems::FlexStart,
                    "center" => AlignItems::Center,
                    "flexEnd" => AlignItems::Auto,
                    "stretch" => AlignItems::Stretch,
                    "baseline" => AlignItems::Baseline,
                    "spaceBetween" => AlignItems::SpaceBetween,
                    "spaceAround" => AlignItems::SpaceAround,
                    &_ => AlignItems::Auto,
                }),
                _ => {}
            }
        }));

        register_float_xml_attribute("paddingTop", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_padding_top(value),
                _ => {}
            }
        }));

        register_float_xml_attribute("paddingRight", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_padding_right(value),
                _ => {}
            }
        }));

        register_float_xml_attribute("paddingBottom", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_padding_bottom(value),
                _ => {}
            }
        }));

        register_float_xml_attribute("paddingLeft", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_padding_left(value),
                _ => {}
            }
        }));

        register_float_xml_attribute("padding", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Box(v) => v.set_padding(value),
                _ => {}
            }
        }));

        s
    }
}

impl BoxView {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let s = Self {
            box_view_data: Default::default(),
            view_data: Default::default(),
        };
        s.set_width(width);
        s.set_height(height);
        s.set_position_top(x);
        s.set_position_left(y);
        s
    }

    pub fn create() -> Rc<RefCell<View>> {
        Rc::new(RefCell::new(View::Box(BoxEnum::Box(BoxView::default()))))
    }
}

impl ViewBase for BoxView {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        &self.view_data
    }
}

impl ViewStyle for BoxView {}

impl ViewLayout for BoxView {}

impl ViewDrawer for BoxView {
    fn draw(&mut self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        BoxTrait::draw(self, ctx, x, y, width, height);
    }
}

impl BoxTrait for BoxView {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        &self.box_view_data
    }
}

pub enum BoxEnum {
    Box(BoxView),
    AppletFrame(AppletFrame),
    BottomBar(BottomBar),
    Button(Button),
    DebugLayer(DebugLayer),
    Dialog(Dialog),
    Dropdown(Dropdown),
    EditTextDialog(EditTextDialog),
    HScrollingFrame(HScrollingFrame),
    Header(Header),
    Hint(Hint),
    Hints(Hints),
    RecyclerCell(RecyclerCell),
    RecyclerHeader(RecyclerHeader),
    ScrollingFrame(ScrollingFrame),
    Slider(Slider),
    TabFrame(TabFrame),
    #[cfg(feature = "mpv")]
    Video(Video),
}

impl ViewTrait for BoxEnum {}

impl ViewDrawer for BoxEnum {
    /// manually dispatch
    fn draw(&mut self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        match self {
            BoxEnum::Box(v) => BoxTrait::draw(v, ctx, x, y, width, height),
            #[cfg(feature = "mpv")]
            BoxEnum::Video(v) => VideoTrait::draw(v, ctx, x, y, width, height),
            _ => {}
        }
    }
}

impl ViewLayout for BoxEnum {}

impl ViewStyle for BoxEnum {}

impl ViewBase for BoxEnum {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        match self {
            BoxEnum::Box(v) => v.view_data(),
            BoxEnum::AppletFrame(v) => v.view_data(),
            #[cfg(feature = "mpv")]
            BoxEnum::Video(v) => v.view_data(),
            BoxEnum::ScrollingFrame(v) => v.view_data(),
            _ => todo!(),
        }
    }
}

pub struct BoxViewData {
    pub axis: Axis,
    pub children: Vec<Rc<RefCell<View>>>,
    pub default_focused_index: usize,
    pub last_focused_view: Option<Rc<RefCell<View>>>,
    pub forwarded_attributes: HashMap<String, (String, Rc<RefCell<RefCell<View>>>)>,
    pub box_view: Option<Rc<RefCell<BoxEnum>>>,
}

impl Default for BoxViewData {
    fn default() -> Self {
        Self {
            axis: Axis::Row,
            children: vec![],
            default_focused_index: 0,
            last_focused_view: None,
            forwarded_attributes: Default::default(),
            box_view: None,
        }
    }
}

impl BoxTrait for BoxEnum {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>> {
        match self {
            BoxEnum::Box(v) => BoxView::box_view_data(v),
            BoxEnum::ScrollingFrame(v) => ScrollingFrame::box_view_data(v),
            _ => todo!(),
        }
    }
}

// Generic FlexBox layout
pub trait BoxTrait: ViewDrawer {
    fn box_view_data(&self) -> &Rc<RefCell<BoxViewData>>;

    /**
     * Adds a view to this Box.
     * Returns the position the view was added at.
     */
    fn add_view(&self, view: Rc<RefCell<View>>) {
        let position = unsafe { YGNodeGetChildCount(self.view_data().borrow().yg_node) };
        self.add_view_position(view, position as usize);
    }

    /**
     * Adds a view to this Box at the given position.
     * Returns the position the view was added at.
     */
    fn add_view_position(&self, view: Rc<RefCell<View>>, position: usize) {
        if position > self.box_view_data().borrow().children.len() {
            panic!(
                "cannot insert view at {}:{}/{}",
                self.describe(),
                self.box_view_data().borrow().children.len(),
                position
            );
        }

        // Add the view to our children and YGNode
        self.box_view_data().borrow_mut()
            .children
            .insert(position, view.clone());

        if !view.borrow().is_detached() {
            unsafe {
                YGNodeInsertChild(
                    self.view_data().borrow().yg_node,
                    view.borrow().view_data().borrow().yg_node,
                    position as u32,
                );
            }
        }

        /// todo: userdata
        view.borrow_mut().set_parent(self.box_view());

        // Layout and events
        self.invalidate();
        view.borrow().will_appear(false);
    }

    /**
     * Removes the given view from the Box. It will be freed.
     */
    fn remove_view(&mut self, to_remove: Rc<RefCell<View>>, free: bool) {
        let mut delete_index = None;
        for (index, view) in self.box_view_data().borrow().children.iter().enumerate() {
            if Rc::ptr_eq(view, &to_remove) {
                delete_index = Some(index)
            }
        }
        if let Some(index) = delete_index {
            let view = self.box_view_data().borrow_mut().children.remove(index);
            // Remove it
            if !view.borrow().is_detached() {
                unsafe {
                    YGNodeRemoveChild(self.view_data().borrow().yg_node, view.borrow().view_data().borrow().yg_node);
                }
            }

            view.borrow().will_disappear(true);
            if free {
                view.borrow_mut().free_view();
            }
            self.invalidate();
        }
    }

    /**
     * Removes all views from the Box. Them will be freed.
     */
    fn clear_views(&mut self, free: bool) {
        let yg_node = self.view_data().borrow().yg_node.clone();
        for view in self.box_view_data().borrow_mut().children.drain(..) {
            // Remove it
            unsafe {
                YGNodeRemoveChild(yg_node, view.borrow().view_data().borrow().yg_node);
            }

            view.borrow().will_disappear(true);
            if free {
                view.borrow_mut().free_view();
            }
        }

        self.invalidate();
    }

    fn on_focus_gained(&mut self) {
        ViewBase::on_focus_gained(self);

        for child in &self.box_view_data().borrow().children {
            child
                .borrow()
                .on_parent_focus_gained(self.view().as_ref().unwrap().clone())
        }
    }

    fn on_focus_lost(&mut self) {
        ViewBase::on_focus_lost(self);

        for child in &self.box_view_data().borrow().children {
            child
                .borrow()
                .on_parent_focus_lost(self.view().as_ref().unwrap().clone())
        }
    }

    fn on_parent_focus_gained(&mut self, focused_view: Rc<RefCell<View>>) {
        ViewBase::on_parent_focus_gained(self, focused_view);

        for child in &self.box_view_data().borrow().children {
            child
                .borrow()
                .on_parent_focus_gained(self.view().as_ref().unwrap().clone())
        }
    }

    fn on_parent_focus_lost(&mut self, focused_view: Rc<RefCell<View>>) {
        ViewBase::on_parent_focus_lost(self, focused_view);

        for child in &self.box_view_data().borrow().children {
            child
                .borrow()
                .on_parent_focus_lost(self.view().as_ref().unwrap().clone())
        }
    }

    fn on_child_focus_gained(
        &mut self,
        direct_child: Rc<RefCell<View>>,
        focused_view: Rc<RefCell<View>>,
    ) {
        self.box_view_data().borrow_mut().last_focused_view = Some(direct_child);
        if let Some(parent) = self.parent() {
            parent
                .borrow_mut()
                .on_child_focus_gained(self.view().unwrap().clone(), focused_view);
        }
    }

    fn on_child_focus_lost(
        &mut self,
        direct_child: Rc<RefCell<View>>,
        focused_view: Rc<RefCell<View>>,
    ) {
        self.box_view_data().borrow_mut().last_focused_view = Some(direct_child);
        if let Some(parent) = self.parent() {
            parent
                .borrow_mut()
                .on_child_focus_lost(self.view().unwrap().clone(), focused_view);
        }
    }

    fn set_padding_full(&self, top: f32, right: f32, bottom: f32, left: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.view_data().borrow().yg_node, YGEdgeTop, top);
            YGNodeStyleSetPadding(self.view_data().borrow().yg_node, YGEdgeRight, right);
            YGNodeStyleSetPadding(self.view_data().borrow().yg_node, YGEdgeBottom, bottom);
            YGNodeStyleSetPadding(self.view_data().borrow().yg_node, YGEdgeLeft, left);
        }
        self.invalidate();
    }

    fn set_padding(&self, padding: f32) {
        self.set_padding_full(padding, padding, padding, padding);
    }

    fn set_padding_top(&self, top: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.view_data().borrow().yg_node, YGEdgeTop, top);
        }
        self.invalidate();
    }

    fn set_padding_right(&self, right: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.view_data().borrow().yg_node, YGEdgeRight, right);
        }
        self.invalidate();
    }

    fn set_padding_bottom(&self, bottom: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.view_data().borrow().yg_node, YGEdgeBottom, bottom);
        }
        self.invalidate();
    }

    fn set_padding_left(&self, left: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.view_data().borrow().yg_node, YGEdgeLeft, left);
        }
        self.invalidate();
    }

    fn padding_top(&self) -> f32 {
        unsafe { YGNodeStyleGetPadding(self.view_data().borrow().yg_node, YGEdgeTop).value }
    }

    fn padding_right(&self) -> f32 {
        unsafe { YGNodeStyleGetPadding(self.view_data().borrow().yg_node, YGEdgeRight).value }
    }

    fn padding_bottom(&self) -> f32 {
        unsafe { YGNodeStyleGetPadding(self.view_data().borrow().yg_node, YGEdgeBottom).value }
    }

    fn padding_left(&self) -> f32 {
        unsafe { YGNodeStyleGetPadding(self.view_data().borrow().yg_node, YGEdgeLeft).value }
    }

    fn default_focus(&self) -> Option<Rc<RefCell<View>>> {
        None
    }

    fn box_view(&self) -> Option<Rc<RefCell<BoxEnum>>> {
        self.box_view_data().borrow().box_view.clone()
    }

    fn set_box_view(&mut self, box_view: Option<Rc<RefCell<BoxEnum>>>) {
        self.box_view_data().borrow_mut().box_view = box_view;
    }

    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        trace!("box draw ({},{},{},{}), childs: {}", x, y, width, height,  &self.box_view_data().borrow().children.len());
        for child in &self.box_view_data().borrow().children {
            trace!("draw {}", child.borrow().describe());
            child.borrow_mut().frame(ctx);
        }
    }

    fn handle_xml_attributes(
        &self,
        element: Node,
        view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
    ) {
        if element.is_element() {
            self.add_view(create_from_xml_element(element, view_creator_registry));
        }
    }

    fn set_axis(&mut self, axis: Axis) {
        unsafe {
            YGNodeStyleSetFlexDirection(self.view_data().borrow().yg_node, get_yg_flex_direction(axis));
        }
        self.box_view_data().borrow_mut().axis = axis;
        self.invalidate();
    }

    fn set_direction(&mut self, direction: Direction) {
        let direction = match direction {
            Direction::Inherit => YGDirection::YGDirectionInherit,
            Direction::LeftToRight => YGDirection::YGDirectionLTR,
            Direction::RightToLeft => YGDirection::YGDirectionRTL,
        };

        unsafe {
            YGNodeStyleSetDirection(self.view_data().borrow().yg_node, direction);
        }

        self.invalidate();
    }

    fn set_justify_content(&mut self, justify_content: JustifyContent) {
        let justify = match justify_content {
            JustifyContent::FlexStart => YGJustify::YGJustifyFlexStart,
            JustifyContent::Center => YGJustify::YGJustifyCenter,
            JustifyContent::FlexEnd => YGJustify::YGJustifyFlexEnd,
            JustifyContent::SpaceBetween => YGJustify::YGJustifySpaceBetween,
            JustifyContent::SpaceAround => YGJustify::YGJustifySpaceAround,
        };
        unsafe {
            YGNodeStyleSetJustifyContent(self.view_data().borrow().yg_node, justify);
        }

        self.invalidate();
    }

    fn set_align_items(&mut self, align_items: AlignItems) {
        let alignment = match align_items {
            AlignItems::Auto => YGAlign::YGAlignAuto,
            AlignItems::FlexStart => YGAlign::YGAlignFlexStart,
            AlignItems::Center => YGAlign::YGAlignCenter,
            AlignItems::FlexEnd => YGAlign::YGAlignFlexEnd,
            AlignItems::Stretch => YGAlign::YGAlignStretch,
            AlignItems::Baseline => YGAlign::YGAlignBaseline,
            AlignItems::SpaceBetween => YGAlign::YGAlignSpaceBetween,
            AlignItems::SpaceAround => YGAlign::YGAlignSpaceAround,
        };

        unsafe {
            YGNodeStyleSetAlignItems(self.view_data().borrow().yg_node, alignment);
        }

        self.invalidate();
    }
}

fn get_yg_flex_direction(axis: Axis) -> YGFlexDirection {
    match axis {
        Axis::Row => YGFlexDirection::YGFlexDirectionRow,
        Axis::Column => YGFlexDirection::YGFlexDirectionColumn
    }
}