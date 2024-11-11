use crate::array_bst::InsertionPlace::NodeAlreadyExists;
use crate::traits::{NodeIdentifiableTree, ParentifiedTree, RotatableTree};
use crate::Tree;

const ROOT_NDX: usize = 0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NodeId {
    node_ndx: usize,
}

#[derive(Default)]
struct Node<Key, Value> {
    key: Key,
    val: Value,
    id: NodeId,
    parent_id: Option<NodeId>,
    left_son_id: Option<NodeId>,
    right_son_id: Option<NodeId>,
}

enum SonType {
    Left,
    Right
}

enum InsertionPlace {
    Root,
    SonOf(NodeId, SonType),
    NodeAlreadyExists(NodeId),
}

pub struct ArrayBST<Key, Value>
where
    Key: Ord,
{
    array: Vec<Node<Key, Value>>,
}

impl<Key, Value> Default for ArrayBST<Key, Value>
where
    Key: Ord,
{
    #[inline(always)]
    fn default() -> Self {
        Self {
            array: Default::default(),
        }
    }
}

impl<Key, Value> ArrayBST<Key, Value>
where
    Key: Ord
{
    fn find_insertion_place(&self, key: &Key) -> InsertionPlace {
        let mut maybe_current_node = self.array.first();

        while let Some(current_node) = maybe_current_node {
            if *key < current_node.key {
                if let Some(left_son_id) = current_node.left_son_id {
                    maybe_current_node = self.get_node_by_id(left_son_id);
                } else {
                    return InsertionPlace::SonOf(current_node.id, SonType::Left);
                }
            } else if *key > current_node.key {
                if let Some(right_son_id) = current_node.right_son_id {
                    maybe_current_node = self.get_node_by_id(right_son_id);
                } else {
                    return InsertionPlace::SonOf(current_node.id, SonType::Right);
                }
            } else {
                return NodeAlreadyExists(current_node.id);
            }
        }

        InsertionPlace::Root
    }

    fn get_node_by_id(&self, node_id: NodeId) -> Option<&Node<Key, Value>> {
        self.array.get(node_id.node_ndx)
    }

    fn get_node_by_id_mut(&mut self, node_id: NodeId) -> Option<&mut Node<Key, Value>> {
        self.array.get_mut(node_id.node_ndx)
    }
}

impl<Key, Value> Tree<Key, Value> for ArrayBST<Key, Value>
where
    Key: Ord,
{
    fn insert(&mut self, key: Key, val: Value) {
        let _ = self.insert_and_get_id(key, val);
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        match self.find_insertion_place(key) {
            InsertionPlace::NodeAlreadyExists(node_id) => {
                self.get_node_by_id(node_id)
                    .map(|node| &node.val)
            },
            _ => {
                None
            },
        }
    }

    fn delete(&mut self, _key: &Key) -> Option<Value> {
        todo!()
    }
}

impl<Key, Value> NodeIdentifiableTree<Key, Value, NodeId> for ArrayBST<Key, Value>
where
    Key: Ord,
{
    fn insert_and_get_id(&mut self, key: Key, val: Value) -> NodeId {
        let inserted_node_id = match self.find_insertion_place(&key) {
            InsertionPlace::Root => {
                let inserted_node_id = NodeId { node_ndx: ROOT_NDX };

                self.array.push(Node {
                    key,
                    val,
                    id: inserted_node_id,
                    parent_id: None,
                    left_son_id: None,
                    right_son_id: None,
                });

                inserted_node_id
            }
            InsertionPlace::SonOf(parent_id, son_type) => {
                let inserted_node_id = NodeId { node_ndx: self.array.len() };

                self.array.push(Node {
                    key,
                    val,
                    id: inserted_node_id,
                    parent_id: Some(parent_id),
                    left_son_id: None,
                    right_son_id: None,
                });

                let parent = self.get_node_by_id_mut(parent_id)
                    .expect("Parent not found");

                match son_type {
                    SonType::Left => {
                        parent.left_son_id = Some(inserted_node_id);
                    }
                    SonType::Right => {
                        parent.right_son_id = Some(inserted_node_id);
                    }
                }

                inserted_node_id
            }
            InsertionPlace::NodeAlreadyExists(node_id) => {
                let node = self.get_node_by_id_mut(node_id).expect("Node not found");
                node.val = val;
                node.id
            }
        };


        println!("inserted_node_id = {:?}", inserted_node_id);

        inserted_node_id
    }

    fn get_by_id(&self, node_id: NodeId) -> Option<&Value> {
        self.get_node_by_id(node_id)
            .map(|node| &node.val)
    }

    fn get_by_id_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_node_by_id_mut(node_id)
            .map(|node| &mut node.val)
    }

    fn get_left_son_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.get_node_by_id(node_id)
            .and_then(|node| node.left_son_id)
    }

    fn get_right_son_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.get_node_by_id(node_id)
            .and_then(|node| node.right_son_id)
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

    fn get_root_mut(&mut self) -> Option<&mut Value> {
        self.array
            .first_mut()
            .map(|node| &mut node.val)
    }

    fn modify<F>(&mut self, node_id: NodeId, mut modifier: F)
    where
        F: FnMut(&mut Value),
    {
        if let Some(node) = self.get_node_by_id_mut(node_id) {
            modifier(&mut node.val);
        }
    }
}

impl<Key, Value> ParentifiedTree<Key, Value, NodeId> for ArrayBST<Key, Value>
where
    Key: Ord,
{
    fn get_parent_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.get_node_by_id(node_id)
            .and_then(|node| node.parent_id)
    }

    fn get_parent(&mut self, node_id: NodeId) -> Option<&Value> {
        self.get_parent_id(node_id)
            .and_then(|parent_id| self.get_by_id(parent_id))
    }

    fn get_parent_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_parent_id(node_id)
            .and_then(|parent_id| self.get_by_id_mut(parent_id))
    }

    fn get_left_uncle_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.get_parent_id(node_id)
            .and_then(|parent_id| self.get_parent_id(parent_id))
            .and_then(|grand_parent_id| self.get_node_by_id(grand_parent_id))
            .and_then(|node| node.left_son_id)
    }

    fn get_right_uncle_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.get_parent_id(node_id)
            .and_then(|parent_id| self.get_parent_id(parent_id))
            .and_then(|grand_parent_id| self.get_node_by_id(grand_parent_id))
            .and_then(|node| node.right_son_id)
    }

    fn get_left_uncle_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_left_uncle_id(node_id)
            .and_then(|uncle_id| self.get_by_id_mut(uncle_id))
    }

    fn get_right_uncle_mut(&mut self, node_id: NodeId) -> Option<&mut Value> {
        self.get_right_uncle_id(node_id)
            .and_then(|uncle_id| self.get_by_id_mut(uncle_id))
    }

    fn is_left_son(&self, node_id: NodeId) -> bool {
        self.get_parent_id(node_id)
            .and_then(|parent_id| self.get_left_son_id(parent_id))
            .filter(|id| *id == node_id)
            .is_some()
    }

    fn is_right_son(&self, node_id: NodeId) -> bool {
        self.get_parent_id(node_id)
            .and_then(|parent_id| self.get_right_son_id(parent_id))
            .filter(|id| *id == node_id)
            .is_some()
    }
}

impl<Key, Value> RotatableTree<Key, Value, NodeId> for ArrayBST<Key, Value>
where
    Key: Ord,
{
    fn right_rotate(&mut self, node_id: NodeId) {
        todo!()
    }

    fn left_rotate(&mut self, node_id: NodeId) {
        todo!()
    }
}

fn stringify_subtree<Key, Value>(
    node_id: NodeId,
    array: &Vec<Node<Key, Value>>,
    prefix: String,
    is_left: bool,
    depth: usize, // Add depth limit to control recursion
) -> String
where
    Key: Ord + std::fmt::Display,
    Value: std::fmt::Display,
{
    // Limit depth to avoid excessive recursion
    if depth > 100 || node_id.node_ndx >= array.len() {
        return format!("{}{} NIL\n", prefix, if is_left { "├──" } else { "└──" });
    }

    let node = &array[node_id.node_ndx];
    let mut result = format!(
        "{}{} {} -> {}\n",
        prefix,
        if is_left { "├──" } else { "└──" },
        node.key,
        node.val
    );

    let child_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });

    if let Some(left_son_id) = node.left_son_id {
        result.push_str(&stringify_subtree(left_son_id, array, child_prefix.clone(), true, depth + 1));
    } else {
        result.push_str(&format!("{}├── NIL\n", child_prefix));
    }

    if let Some(right_son_id) = node.right_son_id {
        result.push_str(&stringify_subtree(right_son_id, array, child_prefix, false, depth + 1));
    } else {
        result.push_str(&format!("{}└── NIL\n", child_prefix));
    }

    result
}

impl<Key, Value> std::fmt::Display for ArrayBST<Key, Value>
where
    Key: Ord + std::fmt::Display,
    Value: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            stringify_subtree(self.get_root_id(), &self.array, "".to_string(), false, 0) // Start with depth 0
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::array_bst::ArrayBST;
    use crate::traits::{NodeIdentifiableTree, ParentifiedTree, RotatableTree, Tree};

    #[test]
    fn test_binary_tree() {
        let mut tree = ArrayBST::<i32, i32>::default();

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
        let mut tree = ArrayBST::default();
        let id = tree.insert_and_get_id(15, 0);
        tree.insert(5, 0);
        tree.insert(1, 0);
        tree.right_rotate(id);
    }
}
