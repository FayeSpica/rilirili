use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Event<T: Clone> {
    callbacks: Rc<RefCell<VecDeque<Box<dyn Fn(T)>>>>,
}

impl<T: Clone + 'static> Event<T> {
    // Creates a new Event
    pub fn new() -> Self {
        Self {
            callbacks: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    // Subscribe to the event with a callback
    pub fn subscribe(&mut self, cb: impl Fn(T) + 'static) -> Rc<RefCell<VecDeque<Box<dyn Fn(T)>>>> {
        self.callbacks.borrow_mut().push_back(Box::new(cb));
        Rc::clone(&self.callbacks)
    }

    // Unsubscribe from the event (removes the last subscription)
    pub fn unsubscribe(&mut self) {
        if !self.callbacks.borrow().is_empty() {
            self.callbacks.borrow_mut().pop_back();
        }
    }

    // Clears all subscriptions
    pub fn clear(&mut self) {
        self.callbacks.borrow_mut().clear();
    }

    // Fires the event, invoking all subscribed callbacks
    pub fn fire(&self, arg: T) -> bool {
        let callbacks = self.callbacks.borrow();
        for cb in callbacks.iter() {
            cb(arg.clone());
        }
        !callbacks.is_empty()
    }
}
