use crate::containers::traits::{
    ErasableContainer, FillableContainer, SearchableContainer, SizedContainer,
};
use crate::subcontainers::traits::*;

pub struct ArrayBasedDeque<T>
where
    T: Default,
{
    ring: Box<[T]>,
    head: usize,
    tail: usize,
    capacity: usize,
    size: usize,
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
    const fn next_ndx(&self, ndx: usize) -> usize {
        let mask = (1 << Self::log2(self.capacity)) - 1;
        (ndx + 1) & mask
    }

    #[inline(always)]
    const fn prev_ndx(&self, ndx: usize) -> usize {
        let mask = (1 << Self::log2(self.capacity)) - 1;
        ndx.wrapping_sub(1) & mask
    }

    pub fn double_the_capacity(&mut self) {
        let new_capacity = self.size() * 2;
        let new_slice = Self::allocate_slice(new_capacity, || T::default());
        let old_slice = std::mem::replace(&mut self.ring, new_slice);

        self.head = 0;
        self.tail = old_slice.len() - 1;
        self.capacity = new_capacity;

        let mut i = 0;
        for element in old_slice.into_vec() {
            self.ring[i] = element;
            i += 1;
        }
    }

    fn allocate_slice<F>(size: usize, mut initializer: F) -> Box<[T]>
    where
        F: FnMut() -> T,
    {
        let layout = std::alloc::Layout::array::<T>(size).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };

        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        for i in 0..size {
            unsafe {
                std::ptr::write(ptr.add(i), initializer());
            }
        }

        unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, size)) }
    }
}

impl<T> Default for ArrayBasedDeque<T>
where
    T: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self {
            ring: Box::new([T::default()]),
            head: 0,
            tail: 1,
            capacity: 1,
            size: 0,
        }
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
        if self.size() == self.capacity {
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
        if self.size() == self.capacity {
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
        for item in self.ring.iter() {
            if predicate(&item) {
                return Some(item);
            }
        }

        None
    }

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut T>
    where
        F: Fn(&T) -> bool,
    {
        for item in self.ring.iter_mut() {
            if predicate(&item) {
                return Some(item);
            }
        }

        None
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
