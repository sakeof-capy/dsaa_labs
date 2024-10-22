#[cfg(test)]
extern crate test;

pub mod queue;
pub mod stack;

pub type ListBasedQueue<T> =
    queue::Queue<T, common::subcontainers::list_based_deque::ListBasedDeque<T>>;
pub type ListBasedStack<T> =
    stack::Stack<T, common::subcontainers::list_based_deque::ListBasedDeque<T>>;
pub type ArrayBasedQueue<T> =
    queue::Queue<T, common::subcontainers::array_based_deque::ArrayBasedDeque<T>>;
pub type ArrayBasedStack<T> =
    stack::Stack<T, common::subcontainers::array_based_deque::ArrayBasedDeque<T>>;

#[cfg(test)]
const BIG_SIZE: usize = 10_000_000;

#[cfg(test)]
mod benchmarks {
    use super::*;
    use common::containers::traits::{FillableContainer, ReadableContainer};
    use test::{black_box, Bencher};

    #[bench]
    fn list_based_stack_pushing(b: &mut Bencher) {
        let mut stack = ListBasedStack::new();

        b.iter(|| {
            black_box(stack.push(1u8));
        });
    }

    #[bench]
    fn array_based_stack_pushing(b: &mut Bencher) {
        let mut stack = ArrayBasedStack::new();

        b.iter(|| {
            black_box(stack.push(1u8));
        });
    }

    #[bench]
    fn list_based_stack_popping(b: &mut Bencher) {
        let mut stack = ListBasedStack::new();

        for i in 0..BIG_SIZE {
            stack.push(i);
        }

        b.iter(|| {
            black_box(stack.pop());
        });
    }

    #[bench]
    fn array_based_stack_popping(b: &mut Bencher) {
        let mut stack = ArrayBasedStack::new();

        for i in 0..BIG_SIZE {
            stack.push(i);
        }

        b.iter(|| {
            black_box(stack.pop());
        });
    }

    #[bench]
    fn list_based_queue_pushing(b: &mut Bencher) {
        let mut queue = ListBasedQueue::new();

        b.iter(|| {
            black_box(queue.push(1u8));
        });
    }

    #[bench]
    fn array_based_queue_pushing(b: &mut Bencher) {
        let mut queue = ArrayBasedQueue::new();

        b.iter(|| {
            black_box(queue.push(1u8));
        });
    }

    #[bench]
    fn list_based_queue_popping(b: &mut Bencher) {
        let mut queue = ListBasedQueue::new();

        for i in 0..BIG_SIZE {
            queue.push(i);
        }

        b.iter(|| {
            black_box(queue.pop());
        });
    }

    #[bench]
    fn array_based_queue_popping(b: &mut Bencher) {
        let mut queue = ArrayBasedQueue::new();

        for i in 0..BIG_SIZE {
            queue.push(i);
        }

        b.iter(|| {
            black_box(queue.pop());
        });
    }
}
