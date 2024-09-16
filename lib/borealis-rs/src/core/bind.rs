use crate::core::view_drawer::ViewTrait;

pub struct BoundView<T: ViewTrait> {
    id: String,
    v: Option<T>,
}

impl<T> BoundView<T>
where
    T: ViewTrait,
{
    pub fn new(id: &str) -> BoundView<T> {
        Self {
            id: id.into(),
            v: None,
        }
    }
}
