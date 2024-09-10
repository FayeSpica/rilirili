use std::cell::RefCell;
use std::rc::Rc;

pub trait ViewBase {
    // Calls draw method, but the actual method depends on the type
    fn frame(&self) {
        // Calls the draw method, which can be overridden
        self.draw();
    }

    // Default draw method
    fn draw(&self) {
        println!("ViewBase::draw");
    }
}

// ViewExtend inherits from ViewBase
pub trait ViewExtend: ViewBase {
    // Override draw in ViewExtend
    fn draw(&self) {
        println!("ViewExtend::draw");
    }
}

// Struct that implements ViewExtend
struct BoxView;

// Implement ViewBase for MyView
impl ViewBase for BoxView {
    fn draw(&self) {
        // Call ViewExtend::draw instead of ViewBase::draw
        ViewExtend::draw(self);
    }
}

// Implement ViewExtend for MyView
impl ViewExtend for BoxView {
}

pub enum View {
    Box(BoxEnum)
}

impl ViewBase for View {
    fn frame(&self) {
        match self { View::Box(v) => ViewBase::frame(v) }
    }

    fn draw(&self) {
        match self { View::Box(v) => ViewExtend::draw(v) }
    }
}

pub enum BoxEnum {
    Box(BoxView)
}

impl ViewBase for BoxEnum {
    fn frame(&self) {
        match self { BoxEnum::Box(v) => ViewBase::frame(v) }
    }

    fn draw(&self) {
        match self { BoxEnum::Box(v) => ViewExtend::draw(v) }
    }
}

impl ViewExtend for BoxEnum {
}

fn main() {
    let my_view = BoxView;

    let r = Rc::new(RefCell::new(View::Box(BoxEnum::Box(my_view))));
    r.borrow().frame();
}
