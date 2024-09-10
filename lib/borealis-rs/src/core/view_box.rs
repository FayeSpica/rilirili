use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use nanovg::Context;
use yoga_sys::{YGNodeGetChildCount, YGNodeInsertChild, YGNodeRemoveChild, YGNodeStyleGetPadding, YGNodeStyleSetPadding};
use yoga_sys::YGEdge::{YGEdgeBottom, YGEdgeLeft, YGEdgeRight, YGEdgeTop};
use crate::core::frame_context::FrameContext;
use crate::core::view_base::{View, ViewBase, ViewData};
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
use crate::views::scrolling_frame::ScrollingFrame;
use crate::views::slider::Slider;
use crate::views::tab_frame::TabFrame;

pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
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
    box_view_data: BoxViewData,
}

impl BoxView {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let s = Self {
            box_view_data: BoxViewData {
                view_data: Default::default(),
                axis: Axis::Row,
                children: vec![],
                default_focused_index: 0,
                last_focused_view: None,
                forwarded_attributes: Default::default(),
                box_view: None,
            }
        };
        s.set_width(width);
        s.set_height(height);
        s.set_position_top(x);
        s.set_position_left(y);
        s
    }
}

impl ViewBase for BoxView {
    fn data(&self) -> &ViewData {
        &self.box_view_data.view_data
    }

    fn data_mut(&mut self) -> &mut ViewData {
        &mut self.box_view_data.view_data
    }
}

impl ViewStyle for BoxView {}

impl ViewLayout for BoxView {}

impl ViewDrawer for BoxView {
    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        BoxTrait::draw(self, ctx, x, y, width, height);
    }
}

impl BoxTrait for BoxView {
    fn box_view_data(&self) -> &BoxViewData {
        &self.box_view_data
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        &mut self.box_view_data
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
}

impl ViewTrait for BoxEnum {}

impl ViewDrawer for BoxEnum {

    /// manually dispatch
    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        match self {
            BoxEnum::Box(v) => BoxTrait::draw(v, ctx, x, y, width, height),
            _ => {}
        }
    }
}

impl ViewLayout for BoxEnum {}

impl ViewStyle for BoxEnum {}

impl ViewBase for BoxEnum {
    fn data(&self) -> &ViewData {
        match self {
            BoxEnum::Box(v) => v.data(),
            BoxEnum::AppletFrame(v) => v.data(),
            _ => todo!(),
        }
    }

    fn data_mut(&mut self) -> &mut ViewData {
        match self {
            BoxEnum::Box(v) => v.data_mut(),
            BoxEnum::AppletFrame(v) => v.data_mut(),
            _ => todo!(),
        }
    }
}

pub struct BoxViewData {
    view_data: ViewData,

    axis: Axis,
    children: Vec<Rc<RefCell<View>>>,

    default_focused_index: usize,
    last_focused_view: Option<Rc<RefCell<View>>>,
    forwarded_attributes: HashMap<String, (String, Rc<RefCell<RefCell<View>>>)>,
    box_view: Option<Rc<RefCell<BoxEnum>>>,
}

impl BoxTrait for BoxEnum {
    fn box_view_data(&self) -> &BoxViewData {
        match self {
            BoxEnum::Box(b) => &b.box_view_data,
            _ => todo!(),
        }
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        match self {
            BoxEnum::Box(b) => &mut b.box_view_data,
            _ => todo!(),
        }
    }
}

// Generic FlexBox layout
pub trait BoxTrait: ViewDrawer {

    fn box_view_data(&self) -> &BoxViewData {
        todo!()
    }

    fn box_view_data_mut(&mut self) -> &mut BoxViewData {
        todo!()
    }

    /**
     * Adds a view to this Box.
     * Returns the position the view was added at.
     */
    fn add_view(&mut self, view: Rc<RefCell<View>>) {
        let position = unsafe {
            YGNodeGetChildCount(self.data().yg_node)
        };
        self.add_view_position(view, position as usize);
    }

    /**
     * Adds a view to this Box at the given position.
     * Returns the position the view was added at.
     */
    fn add_view_position(&mut self, view: Rc<RefCell<View>>, position: usize) {
        if position > self.box_view_data().children.len() {
            panic!("cannot insert view at {}:{}/{}", self.describe(), self.box_view_data().children.len(), position);
        }

        // Add the view to our children and YGNode
        self.box_view_data_mut().children.insert(position, view.clone());

        if !view.borrow().is_detached() {
            unsafe {
                YGNodeInsertChild(self.data().yg_node, view.borrow().data().yg_node, position as u32);
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
        for (index, view)in self.box_view_data().children.iter().enumerate() {
            if Rc::ptr_eq(view, &to_remove) {
                delete_index = Some(index)
            }
        }
        if let Some(index) = delete_index {
            let view = self.box_view_data_mut().children.remove(index);
            // Remove it
            if !view.borrow().is_detached() {
                unsafe {
                    YGNodeRemoveChild(self.data().yg_node, view.borrow().data().yg_node);
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
        let yg_node = self.data().yg_node.clone();
        for view in self.box_view_data_mut().children.drain(..) {
            // Remove it
            unsafe {
                YGNodeRemoveChild(yg_node, view.borrow().data().yg_node);
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

        for child in &self.box_view_data().children {
            child.borrow().on_parent_focus_gained(self.view().as_ref().unwrap().clone())
        }
    }

    fn on_focus_lost(&mut self) {
        ViewBase::on_focus_lost(self);

        for child in &self.box_view_data().children {
            child.borrow().on_parent_focus_lost(self.view().as_ref().unwrap().clone())
        }
    }

    fn on_parent_focus_gained(&mut self, focused_view: Rc<RefCell<View>>) {
        ViewBase::on_parent_focus_gained(self, focused_view);

        for child in &self.box_view_data().children {
            child.borrow().on_parent_focus_gained(self.view().as_ref().unwrap().clone())
        }
    }

    fn on_parent_focus_lost(&mut self, focused_view: Rc<RefCell<View>>) {
        ViewBase::on_parent_focus_lost(self, focused_view);

        for child in &self.box_view_data().children {
            child.borrow().on_parent_focus_lost(self.view().as_ref().unwrap().clone())
        }
    }

    fn on_child_focus_gained(&mut self, direct_child: Rc<RefCell<View>>, focused_view: Rc<RefCell<View>>) {
        self.box_view_data_mut().last_focused_view = Some(direct_child);
        if let Some(parent) = self.parent() {
            parent.borrow_mut().on_child_focus_gained(self.view().unwrap().clone(), focused_view);
        }
    }

    fn on_child_focus_lost(&mut self, direct_child: Rc<RefCell<View>>, focused_view: Rc<RefCell<View>>) {
        self.box_view_data_mut().last_focused_view = Some(direct_child);
        if let Some(parent) = self.parent() {
            parent.borrow_mut().on_child_focus_lost(self.view().unwrap().clone(), focused_view);
        }
    }

    fn set_padding_full(&self, top: f32, right: f32, bottom: f32, left: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.data().yg_node, YGEdgeTop, top);
            YGNodeStyleSetPadding(self.data().yg_node, YGEdgeRight, right);
            YGNodeStyleSetPadding(self.data().yg_node, YGEdgeBottom, bottom);
            YGNodeStyleSetPadding(self.data().yg_node, YGEdgeLeft, left);
        }
        self.invalidate();
    }

    fn set_padding(&self, padding: f32) {
        self.set_padding_full(padding, padding, padding, padding);
    }

    fn set_padding_top(&self, top: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.data().yg_node, YGEdgeTop, top);
        }
        self.invalidate();
    }

    fn set_padding_right(&self, right: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.data().yg_node, YGEdgeRight, right);
        }
        self.invalidate();
    }

    fn set_padding_bottom(&self, bottom: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.data().yg_node, YGEdgeBottom, bottom);
        }
        self.invalidate();
    }

    fn set_padding_left(&self, left: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.data().yg_node, YGEdgeLeft, left);
        }
        self.invalidate();
    }

    fn padding_top(&self) -> f32 {
        unsafe {
            YGNodeStyleGetPadding(self.data().yg_node, YGEdgeTop).value
        }
    }

    fn padding_right(&self) -> f32 {
        unsafe {
            YGNodeStyleGetPadding(self.data().yg_node, YGEdgeRight).value
        }
    }

    fn padding_bottom(&self) -> f32 {
        unsafe {
            YGNodeStyleGetPadding(self.data().yg_node, YGEdgeBottom).value
        }
    }

    fn padding_left(&self) -> f32 {
        unsafe {
            YGNodeStyleGetPadding(self.data().yg_node, YGEdgeLeft).value
        }
    }

    fn default_focus(&self) -> Option<Rc<RefCell<View>>> {
        None
    }

    fn box_view(&self) -> Option<Rc<RefCell<BoxEnum>>> {
        self.box_view_data().box_view.clone()
    }

    fn set_box_view(&mut self, box_view: Option<Rc<RefCell<BoxEnum>>>) {
        self.box_view_data_mut().box_view = box_view;
    }

    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        trace!("box draw {} {} {} {}, childs: {}", x, y, width, height,  &self.box_view_data().children.len());
        for child in &self.box_view_data().children {
            child.borrow().frame(ctx);
        }
    }
}



