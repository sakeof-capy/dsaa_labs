use crate::containers::traits::SizedContainer;
use crate::subcontainers::traits::{BackOrientedContainer, Deque, FrontOrientedContainer};
use std::rc::Rc;

struct Node<T> {
    element: T,
    next_node: Option<Rc<Node<T>>>,
    prev_node: Option<Rc<Node<T>>>,
}

pub struct ListBasedDeque<T> {
    size: usize,
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
}

impl<T> ListBasedDeque<T> {
    pub fn new() -> Self {
        Default::default()
    }

    unsafe fn modify_node(node: &Option<Rc<Node<T>>>, mut f: impl FnMut(*mut Node<T>)) {
        if let Some(ref next) = node {
            let next_ptr = Rc::as_ptr(next) as *mut Node<T>;
            f(next_ptr);
        }
    }
}

impl<T> Default for ListBasedDeque<T> {
    fn default() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }
}

impl<T> SizedContainer for ListBasedDeque<T> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.size
    }
}

impl<T> FrontOrientedContainer<T> for ListBasedDeque<T> {
    fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.element)
    }

    fn push_front(&mut self, element: T) {
        let new_head = Rc::new(Node {
            element,
            next_node: self.head.take(),
            prev_node: None,
        });

        unsafe {
            Self::modify_node(&new_head.next_node, |next_ptr| {
                (*next_ptr).prev_node = Some(new_head.clone())
            });
        }

        self.head = Some(new_head.clone());

        if self.tail.is_none() {
            self.tail = Some(new_head);
        }

        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            unsafe {
                Self::modify_node(&old_head.next_node, |next_ptr| {
                    (*next_ptr).prev_node.take();
                });
            }

            if let Some(new_head) = old_head.next_node.as_ref() {
                self.head = Some(new_head.clone());
            } else {
                self.tail = None;
            }

            self.size -= 1;

            Rc::try_unwrap(old_head).ok().unwrap().element
        })
    }
}

impl<T> BackOrientedContainer<T> for ListBasedDeque<T> {
    fn push_back(&mut self, element: T) {
        let new_tail = Rc::new(Node {
            element,
            next_node: None,
            prev_node: self.tail.take(),
        });

        unsafe {
            Self::modify_node(&new_tail.prev_node, |next_ptr| {
                (*next_ptr).next_node = Some(new_tail.clone())
            });
        }

        self.tail = Some(new_tail.clone());

        if self.head.is_none() {
            self.head = Some(new_tail);
        }

        self.size += 1;
    }

    fn back(&self) -> Option<&T> {
        self.tail.as_ref().map(|tail| &tail.element)
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            unsafe {
                Self::modify_node(&old_tail.prev_node, |next_ptr| {
                    (*next_ptr).next_node.take();
                });
            }

            if let Some(new_tail) = old_tail.prev_node.as_ref() {
                self.tail = Some(new_tail.clone());
            } else {
                self.head = None;
            }

            self.size -= 1;

            Rc::try_unwrap(old_tail).ok().unwrap().element
        })
    }
}

impl<T> Deque<T> for ListBasedDeque<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subcontainers::list_based_deque::ListBasedDeque;

    #[test]
    fn deque_test() {
        let mut deque = ListBasedDeque::new();

        for i in 0..10 {
            deque.push_front(i);
        }

        for i in -9..0 {
            deque.push_back(i);
        }

        let mut size = 19;
        while let Some(_) = deque.pop_back() {
            size -= 1;
            assert_eq!(deque.size(), size);
        }
    }
}
