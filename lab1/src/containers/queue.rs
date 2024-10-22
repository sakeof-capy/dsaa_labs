use common::containers::traits::{FillableContainer, ReadableContainer, SizedContainer};
use common::subcontainers::traits::Deque;
use std::marker::PhantomData;

pub struct Queue<Element, Subcontainer>
where
    Subcontainer: Default + Deque<Element>,
{
    subcontainer: Subcontainer,
    _marker: PhantomData<Element>,
}

impl<Element, Subcontainer> Queue<Element, Subcontainer>
where
    Subcontainer: Default + Deque<Element>,
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<Element, Subcontainer> Default for Queue<Element, Subcontainer>
where
    Subcontainer: Default + Deque<Element>,
{
    fn default() -> Self {
        Self {
            subcontainer: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<Element, Subcontainer> FillableContainer<Element> for Queue<Element, Subcontainer>
where
    Subcontainer: Default + Deque<Element>,
{
    fn push(&mut self, element: Element) -> &mut Self {
        self.subcontainer.push_back(element);
        self
    }
}

impl<Element, Subcontainer> SizedContainer for Queue<Element, Subcontainer>
where
    Subcontainer: Default + Deque<Element>,
{
    fn size(&self) -> usize {
        self.subcontainer.size()
    }
}

impl<Element, Subcontainer> ReadableContainer<Element> for Queue<Element, Subcontainer>
where
    Subcontainer: Default + Deque<Element>,
{
    fn pivot(&self) -> Option<&Element> {
        self.subcontainer.front()
    }

    fn pop(&mut self) -> Option<Element> {
        self.subcontainer.pop_front()
    }
}
