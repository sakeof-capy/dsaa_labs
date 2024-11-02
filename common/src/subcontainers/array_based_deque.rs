use crate::containers::traits::{
    ErasableContainer, FillableContainer, SearchableContainer, SizedContainer,
};
use crate::subcontainers::resizable_array::ResizableArray;
use crate::subcontainers::traits::*;

pub struct ArrayBasedDeque<T>
where
    T: Default,
{
    ring: ResizableArray<T>,
    head: usize,
    tail: usize,
    size: usize,
}


impl<T> Default for ArrayBasedDeque<T>
where
    T: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self {
            ring: ResizableArray::new(1),
            head: 0,
            tail: 1,
            size: 0,
        }
    }
}

impl<T> ArrayBasedDeque<T>
where
    T: Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    const fn log2(n: usize) -> usize {
        usize::BITS as usize - 1 - n.leading_zeros() as usize
    }

    #[inline(always)]
    fn next_ndx(&self, ndx: usize) -> usize {
        let mask = (1 << Self::log2(self.capacity())) - 1;
        (ndx + 1) & mask
    }

    #[inline(always)]
    fn prev_ndx(&self, ndx: usize) -> usize {
        let mask = (1 << Self::log2(self.capacity())) - 1;
        ndx.wrapping_sub(1) & mask
    }

    fn double_the_capacity(&mut self) {
        self.ring.double_the_size();
    }

    fn capacity(&self) -> usize {
        self.ring.size()
    }
}

impl<T> SizedContainer for ArrayBasedDeque<T>
where
    T: Default,
{
    #[inline(always)]
    fn size(&self) -> usize {
        self.size
    }
}

impl<T> FrontOrientedContainer<T> for ArrayBasedDeque<T>
where
    T: Default,
{
    fn push_front(&mut self, element: T) {
        if self.size() == self.capacity() {
            self.double_the_capacity();
        }

        self.head = self.prev_ndx(self.head);
        self.ring[self.head] = element;
        self.size += 1;
    }

    #[inline(always)]
    fn pop_front(&mut self) -> Option<T> {
        (self.size() > 0).then(|| {
            let prev_head = std::mem::take(&mut self.ring[self.head]);
            self.head = self.next_ndx(self.head);
            self.size -= 1;
            prev_head
        })
    }

    #[inline(always)]
    fn front(&self) -> Option<&T> {
        (self.size() > 0).then(|| &self.ring[self.head])
    }
}

impl<T> BackOrientedContainer<T> for ArrayBasedDeque<T>
where
    T: Default,
{
    fn push_back(&mut self, element: T) {
        if self.size() == self.capacity() {
            self.double_the_capacity();
        }

        self.tail = self.next_ndx(self.tail);
        self.ring[self.tail] = element;
        self.size += 1;
    }

    #[inline(always)]
    fn back(&self) -> Option<&T> {
        (self.size() > 0).then(|| &self.ring[self.tail])
    }

    #[inline(always)]
    fn pop_back(&mut self) -> Option<T> {
        (self.size() > 0).then(|| {
            let prev_tail = std::mem::take(&mut self.ring[self.tail]);
            self.tail = self.prev_ndx(self.tail);
            self.size -= 1;
            prev_tail
        })
    }
}

impl<T> Deque<T> for ArrayBasedDeque<T> where T: Default {}

impl<T> ErasableContainer<T> for ArrayBasedDeque<T>
where
    T: Default,
{
    fn erase_first<F>(&mut self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.find_mut(predicate).map(|found| std::mem::take(found))
    }
}

impl<T> FillableContainer<T> for ArrayBasedDeque<T>
where
    T: Default,
{
    fn push(&mut self, element: T) -> &mut Self {
        self.push_back(element);
        self
    }
}

impl<T> SearchableContainer<T> for ArrayBasedDeque<T>
where
    T: Default,
{
    fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        self.ring.find(predicate)
    }

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut T>
    where
        F: Fn(&T) -> bool,
    {
        self.ring.find_mut(predicate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let end = 10;
        let mut deque = ArrayBasedDeque::new();

        for i in 0..end {
            deque.push_back(i);
            assert_eq!(deque.size(), i + 1);
            assert!(deque.back().is_some());
            assert_eq!(*deque.back().unwrap(), i);
            assert!(deque.front().is_some());
            assert_eq!(*deque.front().unwrap(), 0);
        }

        for i in 0..(end / 2) {
            assert!(deque.pop_front().is_some());
            assert_eq!(deque.size(), end - i - 1);
            assert!(deque.back().is_some());
            assert_eq!(*deque.back().unwrap(), end - 1);
            assert!(deque.front().is_some());
            assert_eq!(*deque.front().unwrap(), i + 1);
        }

        for i in (0..(end / 2)).rev() {
            deque.push_front(i);
            assert_eq!(deque.size(), end - i);
            assert!(deque.back().is_some());
            assert_eq!(*deque.back().unwrap(), end - 1);
            assert!(deque.front().is_some());
            assert_eq!(*deque.front().unwrap(), i);
        }

        for i in 0..(end - 1) {
            let popped = deque.pop_back();
            assert!(popped.is_some());
            assert_eq!(popped.unwrap(), end - i - 1);
            assert_eq!(deque.size(), end - i - 1);
            assert!(deque.back().is_some());
            assert_eq!(*deque.back().unwrap(), end - i - 2);
            assert!(deque.front().is_some());
            assert_eq!(*deque.front().unwrap(), 0);
        }

        let popped = deque.pop_front();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap(), 0);
        assert!(deque.back().is_none());
        assert!(deque.front().is_none());
        assert_eq!(deque.size(), 0);
    }
}
