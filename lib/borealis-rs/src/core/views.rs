use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use crate::core::view_base::View;

lazy_static! {
    static ref VIEW_MAP: Mutex<Views> = Mutex::new(Views::default());
}

pub struct Views {
    view_map: HashMap<String, Rc<RefCell<View>>>
}

impl Default for Views {
    fn default() -> Self {
        Self {
            view_map: HashMap::new(),
        }
    }
}

unsafe impl Send for Views{}
unsafe impl Sync for Views{}

pub fn get_view(id: &str) -> Option<Rc<RefCell<View>>>{
    let map = VIEW_MAP.lock().unwrap();
    map.view_map.get(id).cloned()
}

pub fn add_view(id: &str, view: Rc<RefCell<View>>){
    let mut map = VIEW_MAP.lock().unwrap();
    map.view_map.insert(id.into(), view);
}