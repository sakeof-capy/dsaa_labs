pub trait SizedContainer {
    fn size(&self) -> usize;
}

pub trait FillableContainer<T> {
    fn push(&mut self, element: T) -> &mut Self;
}

pub trait ErasableContainer<T> {
    fn erase_first<F>(&mut self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool;
}

pub trait ReadableContainer<T>: SizedContainer {
    fn pivot(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
}

pub trait SearchableContainer<T> {
    fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool;

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut T>
    where
        F: Fn(&T) -> bool;
}
