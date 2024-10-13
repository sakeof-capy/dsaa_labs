pub trait SizedContainer {
    fn size(&self) -> usize;
}

pub trait FillableContainer<T> {
    fn push(&mut self, element: T) -> &mut Self;
}

pub trait ReadableContainer<T>: SizedContainer {
    fn pivot(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
}

pub trait Hasher<T> {
    fn hash(state: &mut T);
}
