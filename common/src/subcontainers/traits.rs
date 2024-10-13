use crate::containers::traits::SizedContainer;

pub trait FrontOrientedContainer<T>: SizedContainer {
    fn push_front(&mut self, element: T);
    fn pop_front(&mut self) -> Option<T>;
    fn front(&self) -> Option<&T>;
}

pub trait BackOrientedContainer<T>: SizedContainer {
    fn push_back(&mut self, element: T);
    fn back(&self) -> Option<&T>;
    fn pop_back(&mut self) -> Option<T>;
}

pub trait Deque<T>: FrontOrientedContainer<T> + BackOrientedContainer<T> {}
