use common::containers::traits::{SearchableContainer, SizedContainer};
use common::subcontainers::resizable_array::ResizableArray;
use crate::traits::Tree;

const ROOT_NDX: usize = 1;
const INITIAL_CAPACITY: usize = ROOT_NDX + 1;

#[derive(Default)]
struct Node<Key, Value> {
    key: Key,
    val: Value,
}

type Cell<Key, Value> = Option<Node<Key, Value>>;

pub struct ArrayBinarySearchTree<Key, Value>
where
    Key: Ord,
{
    array: ResizableArray<Cell<Key, Value>>,
}

impl<Key, Value> Default for ArrayBinarySearchTree<Key, Value>
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

impl<Key, Value> ArrayBinarySearchTree<Key, Value>
where
    Key: Ord,
{
    fn find_index(&self, key: &Key) -> usize {
        let mut index = ROOT_NDX;

        while index < self.array.size()
            && let Some(node) = &self.array[index]
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

impl<Key, Value> Tree<Key, Value> for ArrayBinarySearchTree<Key, Value>
where
    Key: Ord,
{
    fn insert(&mut self, key: Key, val: Value) {
        match self.find_cell_mut(&key) {
            Ok(Some(node)) => {
                node.val = val;
            }
            Ok(cell) => {
                *cell = Some(Node { key, val });
            }
            Err(next_index) => {
                self.array.double_the_size();
                self.array[next_index] = Some(Node { key, val });
            }
        }
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        match self.find_cell(key) {
            Ok(Some(node)) => Some(&node.val),
            _ => None,
        }
    }

    fn delete(&mut self, key: &Key) -> Option<Value> {
        // self.find_cell_mut(key)
        //     .ok()
        //     .and_then(|cell| std::mem::take(cell))
        //     .map(|node| node.val)
        todo!()
    }
}

fn stringify_subtree<Key, Value>(
    root_ndx: usize,
    array: &ResizableArray<Cell<Key, Value>>,
    prefix: String,
    is_left: bool,
) -> String
where
    Key: Ord + std::fmt::Display,
    Value: std::fmt::Display,
{
    if root_ndx >= array.size() || array[root_ndx].is_none() {
        return format!("{}{} NIL\n", prefix, if is_left { "├──" } else { "└──" });
    }

    let node = array[root_ndx].as_ref().unwrap();
    let mut result = format!(
        "{}{} {} -> {}\n",
        prefix,
        if is_left { "├──" } else { "└──" },
        node.key,
        node.val
    );

    let child_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
    let left_child = 2 * root_ndx;
    let right_child = 2 * root_ndx + 1;

    result.push_str(&stringify_subtree(left_child, array, child_prefix.clone(), true));
    result.push_str(&stringify_subtree(right_child, array, child_prefix, false));

    result
}

impl<Key, Value> std::fmt::Display for ArrayBinarySearchTree<Key, Value>
where
    Key: Ord + std::fmt::Display,
    Value: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", stringify_subtree(ROOT_NDX, &self.array, "".to_string(), false))
    }
}

#[cfg(test)]
mod tests {
    use crate::array_binary_search_tree::ArrayBinarySearchTree;
    use crate::traits::Tree;

    #[test]
    fn test_binary_tree() {
        let mut tree = ArrayBinarySearchTree::<i32, i32>::default();

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
