use std::array;
use common::containers::traits::{SizedContainer, SearchableContainer};
use common::subcontainers::resizable_array::ResizableArray;

const ROOT_NDX: usize = 1;
const INITIAL_CAPACITY: usize = ROOT_NDX + 1;

type Color = bool;

const RED: Color = true;
const BLACK: Color = false;

#[derive(Default)]
struct Node<Key, Value> {
    key: Key,
    val: Value,
    color: Color,
}

type Cell<Key, Value> = Option<Node<Key, Value>>;

pub struct BinaryTree<Key, Value>
where
    Key: Ord,
{
    array: ResizableArray<Cell<Key, Value>>,
}

impl<Key, Value> Default for BinaryTree<Key, Value>
where
    Key: Ord,
{
    #[inline(always)]
    fn default() -> Self {
        Self {
            array: ResizableArray::new(INITIAL_CAPACITY),
        }
    }
}

impl<Key, Value> BinaryTree<Key, Value>
where
    Key: Ord,
{
    pub fn insert(&mut self, key: Key, val: Value) {
        match self.find_cell_mut(&key) {
            Ok(Some(node)) => {
                node.val = val;
            }
            Ok(cell) => {
                *cell = Some(Node { key, val, color: RED });
            }
            Err(next_index) => {
                self.array.double_the_size();
                self.array[next_index] = Some(Node { key, val, color: RED });
            }
        }
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        match self.find_cell(&key) {
            Ok(Some(node)) => {
                Some(&node.val)
            }
            _ => None,
        }
    }

    fn find_index(&self, key: &Key) -> usize {
        let mut index = ROOT_NDX;

        while index < self.array.size() &&
            let Some(node) = &self.array[index]
        {
            if &node.key < key {
                index *= 2;
            } else if &node.key > key {
                index = 2 * index + 1;
            } else {
                break;
            }
        }

        index
    }

    fn find_cell(&self, key: &Key) -> Result<&Cell<Key, Value>, usize> {
        let index = self.find_index(key);
        (index < self.array.size())
            .then(|| &self.array[index])
            .ok_or(index)
    }

    fn find_cell_mut(&mut self, key: &Key) -> Result<&mut Cell<Key, Value>, usize> {
        let index = self.find_index(key);
        (index < self.array.size())
            .then(|| &mut self.array[index])
            .ok_or(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::BinaryTree;

    #[test]
    fn test_binary_tree() {
        let mut tree = BinaryTree::<i32, i32>::default();

        let pairs = [
            (0, 1),
            (1, 2),
            (-1, 3),
            (2, 3),
            (3, 4),
            (-10, 1010),
            (4, 5),
            (-321, 5),
            (5, 6),
            (6, 7),
            (7, 8),
        ];

        for (key, val) in &pairs {
            tree.insert(*key, *val);
        }

        for (key, val) in pairs {
            let found = tree.get(&key);
            assert!(found.is_some());
            assert_eq!(*found.unwrap(), val);
        }

        for non_existent in 1000..2000 {
            let found = tree.get(&non_existent);
            assert!(found.is_none());
        }
    }
}
