use crate::containers::traits::SearchableContainer;
use crate::containers::traits::SizedContainer;
use std::ops::{Index, IndexMut};

pub struct ResizableArray<T>
where
    T: Default,
{
    array: Box<[T]>,
}

impl<T> Default for ResizableArray<T>
where
    T: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new(0)
    }
}

impl<T> ResizableArray<T>
where
    T: Default,
{
    #[inline(always)]
    pub fn new(initial_size: usize) -> Self {
        Self {
            array: Self::allocate_slice(initial_size, || T::default()),
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        let new_slice = Self::allocate_slice(new_size, || T::default());
        let old_slice = std::mem::replace(&mut self.array, new_slice);

        let mut i = 0;
        for element in old_slice.into_vec() {
            self.array[i] = element;
            i += 1;
        }
    }

    pub fn double_the_size(&mut self) {
        self.resize(self.size() * 2);
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

impl<T> Index<usize> for ResizableArray<T>
where
    T: Default,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<T> IndexMut<usize> for ResizableArray<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl<T> SizedContainer for ResizableArray<T>
where
    T: Default,
{
    #[inline(always)]
    fn size(&self) -> usize {
        self.array.len()
    }
}

impl<T> SearchableContainer<T> for ResizableArray<T>
where
    T: Default,
{
    fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        for item in self.array.iter() {
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
        for item in self.array.iter_mut() {
            if predicate(&item) {
                return Some(item);
            }
        }

        None
    }
}
