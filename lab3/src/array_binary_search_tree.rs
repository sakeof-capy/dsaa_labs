use crate::traits::{NodeIdentifiableTree, ParentifiedTree, RotatableTree, Tree};
use common::containers::traits::{SearchableContainer, SizedContainer};
use common::subcontainers::resizable_array::ResizableArray;

const ROOT_NDX: usize = 1;
const INITIAL_CAPACITY: usize = ROOT_NDX + 1;

#[derive(Default)]
struct Node<Key, Value> {
    key: Key,
    val: Value,
}

type Cell<Key, Value> = Option<Node<Key, Value>>;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct NodeId {
    node_ndx: usize,
}

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

    fn find_cell_mut(&mut self, key: &Key) -> Result<(&mut Cell<Key, Value>, usize), usize> {
        let index = self.find_index(key);
        (index < self.array.size())
            .then(|| (&mut self.array[index], index))
            .ok_or(index)
    }

    fn extract_node_with_id(&self, node_id: NodeId) -> bool {
        node_id.node_ndx < self.array.size() && self.array[node_id.node_ndx].is_some()
    }
}

impl<Key, Value> Tree<Key, Value> for ArrayBinarySearchTree<Key, Value>
where
    Key: Ord,
{
    fn insert(&mut self, key: Key, val: Value) {
        let _ = self.insert_and_get_id(key, val);
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

impl<Key, Value> NodeIdentifiableTree<Key, Value, NodeId> for ArrayBinarySearchTree<Key, Value>
where
    Key: Ord,
{
    fn insert_and_get_id(&mut self, key: Key, val: Value) -> NodeId {
        let node_ndx = match self.find_cell_mut(&key) {
            Ok((Some(node), node_ndx)) => {
                node.val = val;
                node_ndx
            }
            Ok((cell, node_ndx)) => {
                *cell = Some(Node { key, val });
                node_ndx
            }
            Err(next_index) => {
                self.array.double_the_size();
                self.array[next_index] = Some(Node { key, val });
                next_index
            }
        };

        NodeId { node_ndx }
    }

    fn get_by_id(&self, node_id: NodeId) -> Option<&Value> {
        if node_id.node_ndx >= self.array.size() {
            return None;
        }

        match self.array[node_id.node_ndx] {
            Some(ref node) => Some(&node.val),
            None => None,
        }
    }

    fn get_by_id_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        match self.array[node_id.node_ndx] {
            Some(ref mut node) => Some(&mut node.val),
            None => None,
        }
    }

    fn get_left_son_id(&self, node_id: NodeId) -> Option<NodeId> {
        let _ = self.get_by_id(node_id)?;
        let right_son_ndx = 2 * node_id.node_ndx + 1;
        self.get_by_id(NodeId {
            node_ndx: right_son_ndx,
        })
        .map(|_| NodeId {
            node_ndx: right_son_ndx,
        })
    }

    fn get_right_son_id(&self, node_id: NodeId) -> Option<NodeId> {
        let _ = self.get_by_id(node_id)?;
        let right_son_ndx = 2 * node_id.node_ndx;
        self.get_by_id(NodeId {
            node_ndx: right_son_ndx,
        })
        .map(|_| NodeId {
            node_ndx: right_son_ndx,
        })
    }

    fn get_left_uncle_id(&self, node_id: NodeId) -> Option<NodeId> {
        let parent_id = self.get_parent_id(node_id)?;
        let grand_parent_id = self.get_parent_id(parent_id)?;
        self.get_left_son_id(grand_parent_id)
    }

    fn get_right_uncle_id(&self, node_id: NodeId) -> Option<NodeId> {
        let parent_id = self.get_parent_id(node_id)?;
        let grand_parent_id = self.get_parent_id(parent_id)?;
        self.get_right_son_id(grand_parent_id)
    }

    fn get_root_id(&self) -> NodeId {
        NodeId { node_ndx: ROOT_NDX }
    }

    fn get_left_son_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_by_id_mut(self.get_left_son_id(node_id)?)
    }

    fn get_right_son_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_by_id_mut(self.get_right_son_id(node_id)?)
    }

    fn get_left_uncle_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_by_id_mut(self.get_left_uncle_id(node_id)?)
    }

    fn get_right_uncle_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_by_id_mut(self.get_right_uncle_id(node_id)?)
    }

    fn get_root_mut(&mut self) -> Option<&mut Value> {
        self.get_by_id_mut(self.get_root_id())
    }

    fn modify<F>(&mut self, node_id: NodeId, mut modifier: F)
    where
        F: FnMut(&mut Value),
    {
        if let Some(value) = self.get_by_id_mut(node_id) {
            modifier(value);
        }
    }
}

impl<Key, Value> ParentifiedTree<Key, Value, NodeId> for ArrayBinarySearchTree<Key, Value>
where
    Key: Ord,
{
    fn get_parent_id(&self, node_id: NodeId) -> Option<NodeId> {
        let _ = self.get_by_id(node_id)?;
        let parent_ndx = node_id.node_ndx / 2;
        self.get_by_id(NodeId {
            node_ndx: parent_ndx,
        })
        .map(|_| NodeId {
            node_ndx: parent_ndx,
        })
    }

    fn get_parent(&mut self, node_id: NodeId) -> Option<&Value> {
        self.get_by_id(self.get_parent_id(node_id)?)
    }

    fn get_parent_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_by_id_mut(self.get_parent_id(node_id)?)
    }

    fn is_left_son(&self, node_id: NodeId) -> bool {
        if let Some(parent_id) = self.get_parent_id(node_id)
            && let Some(left_brother_id) = self.get_left_son_id(parent_id)
        {
            node_id == left_brother_id
        } else {
            false
        }
    }

    fn is_right_son(&self, node_id: NodeId) -> bool {
        if let Some(parent_id) = self.get_parent_id(node_id)
            && let Some(right_brother_id) = self.get_right_son_id(parent_id)
        {
            parent_id == right_brother_id
        } else {
            false
        }
    }
}

impl<Key, Value> RotatableTree<Key, Value, NodeId> for ArrayBinarySearchTree<Key, Value>
where
    Key: Ord,
{
    fn right_rotate(&mut self, node_id: NodeId) {
        println!("RIGHT ROTATE");
    }

    fn left_rotate(&mut self, node_id: NodeId) {
        println!("LEFT ROTATE");
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

    result.push_str(&stringify_subtree(
        left_child,
        array,
        child_prefix.clone(),
        true,
    ));
    result.push_str(&stringify_subtree(right_child, array, child_prefix, false));

    result
}

impl<Key, Value> std::fmt::Display for ArrayBinarySearchTree<Key, Value>
where
    Key: Ord + std::fmt::Display,
    Value: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            stringify_subtree(ROOT_NDX, &self.array, "".to_string(), false)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::array_binary_search_tree::ArrayBinarySearchTree;
    use crate::traits::{NodeIdentifiableTree, ParentifiedTree, RotatableTree, Tree};

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

    #[test]
    fn test_rotations() {
        let mut tree = ArrayBinarySearchTree::default();
        let id = tree.insert_and_get_id(15, 0);
        tree.insert(5, 0);
        tree.insert(1, 0);
        tree.right_rotate(id);
    }
}
