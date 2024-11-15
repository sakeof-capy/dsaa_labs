use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::traits::{NodeIdentifiableTree, ParentifiedTree, RotatableTree, Tree};

#[derive(Clone)]
pub(crate) struct NodeId<Key, Value> {
    node: Weak<RefCell<Node<Key, Value>>>
}

impl<Key, Value> NodeId<Key, Value> {
    fn new(node: Rc<RefCell<Node<Key, Value>>>) -> Self {
        Self {
            node: Rc::downgrade(&node)
        }
    }
}

struct Node<Key, Value> {
    key: Key,
    val: Value,
    // id: NodeId<Key, Value>,
    parent_id: Option<NodeId<Key, Value>>,
    left_son_id: Option<Rc<RefCell<Node<Key, Value>>>>,
    right_son_id: Option<Rc<RefCell<Node<Key, Value>>>>,
}

enum SonType {
    Left,
    Right,
}

enum InsertionPlace<Key, Value> {
    Root,
    SonOf(NodeId<Key, Value>, SonType),
    NodeAlreadyExists(NodeId<Key, Value>),
}

#[derive(Default)]
pub struct ListBST<Key, Value>
where
    Key: Ord,
    Value: Clone,
{
    root: Option<Rc<RefCell<Node<Key, Value>>>>,
}

impl<Key, Value> ListBST<Key, Value>
where
    Key: Ord,
    Value: Clone,
{
    fn find_insertion_place(&self, key: &Key) -> InsertionPlace<Key, Value> {
        let mut maybe_current_node = self.root.clone();

        while let Some(current_node) = maybe_current_node.clone() {
            if *key < current_node.borrow().key {
                if let Some(left_son) = current_node.borrow().left_son_id.clone() {
                    maybe_current_node = Some(left_son);
                } else {
                    return InsertionPlace::SonOf(NodeId::new(current_node.clone()), SonType::Left);
                }
            } else if *key > current_node.borrow().key {
                if let Some(right_son) = current_node.borrow().right_son_id.clone() {
                    maybe_current_node = Some(right_son);
                } else {
                    return InsertionPlace::SonOf(NodeId::new(current_node.clone()), SonType::Right);
                }
            } else {
                return InsertionPlace::NodeAlreadyExists(NodeId::new(current_node.clone()));
            }
        }

        InsertionPlace::Root
    }
}

impl<Key, Value> Tree<Key, Value> for ListBST<Key, Value>
where
    Key: Ord,
    Value: Clone,
{
    fn insert(&mut self, key: Key, val: Value) {
        let _ = self.insert_and_get_id(key, val);
    }

    fn get(&self, key: &Key) -> Option<Value> {
        match self.find_insertion_place(key) {
            InsertionPlace::NodeAlreadyExists(node_id) => {
                node_id.node
                    .upgrade()
                    .map(|node| node.borrow().val.clone())
            }
            _ => None,
        }
    }

    fn delete(&mut self, key: &Key) -> Option<Value> {
        todo!()
    }
}

impl<Key, Value> NodeIdentifiableTree<Key, Value, NodeId<Key, Value>> for ListBST<Key, Value>
where
    Key: Ord,
    Value: Clone,
{
    fn insert_and_get_id(&mut self, key: Key, val: Value) -> NodeId<Key, Value> {
        let inserted_node_id = match self.find_insertion_place(&key) {
            InsertionPlace::Root => {
                self.root = Some(Rc::new(RefCell::new(Node {
                    key,
                    val,
                    parent_id: None,
                    left_son_id: None,
                    right_son_id: None,
                })));

                NodeId {
                    node: Rc::downgrade(&self.root.clone().unwrap())
                }
            }
            InsertionPlace::SonOf(parent_id, son_type) => {
                let parent = parent_id.node.upgrade().expect("This Rc must live longer!");
                let new_node = Rc::new(RefCell::new(Node {
                    key,
                    val,
                    parent_id: Some(parent_id),
                    left_son_id: None,
                    right_son_id: None,
                }));

                match son_type {
                    SonType::Left => {
                        parent.borrow_mut().left_son_id = Some(new_node.clone());
                    }
                    SonType::Right => {
                        parent.borrow_mut().right_son_id = Some(new_node.clone());
                    }
                }

                NodeId {
                    node: Rc::downgrade(&new_node)
                }
            }
            InsertionPlace::NodeAlreadyExists(node_id) => {
                let node = node_id.node.upgrade().expect("This Rc must live longer!");
                node.borrow_mut().key = key;
                node.borrow_mut().val = val;
                node_id
            }
        };

        inserted_node_id
    }

    fn get_by_id(&self, node_id: NodeId<Key, Value>) -> Option<&Value> {
        todo!()
    }

    fn get_by_id_mut(&mut self, node_id: NodeId<Key, Value>) -> Option<&mut Value> {
        todo!()
    }

    fn get_left_son_id(&self, node_id: NodeId<Key, Value>) -> Option<NodeId<Key, Value>> {
        todo!()
    }

    fn get_right_son_id(&self, node_id: NodeId<Key, Value>) -> Option<NodeId<Key, Value>> {
        todo!()
    }

    fn get_root_id(&self) -> Option<NodeId<Key, Value>> {
        todo!()
    }

    fn get_left_son_mut(&mut self, node_id: NodeId<Key, Value>) -> Option<&mut Value> {
        todo!()
    }

    fn get_right_son_mut(&mut self, node_id: NodeId<Key, Value>) -> Option<&mut Value> {
        todo!()
    }

    fn get_root_mut(&mut self) -> Option<&mut Value> {
        todo!()
    }

    fn modify<F>(&mut self, node_id: NodeId<Key, Value>, modifier: F)
    where
        F: FnMut(&mut Value)
    {
        todo!()
    }
}

impl<Key, Value> ParentifiedTree<Key, Value, NodeId<Key, Value>> for ListBST<Key, Value>
where
    Key: Ord,
    Value: Clone,
{
    fn get_parent_id(&self, node_id: NodeId<Key, Value>) -> Option<NodeId<Key, Value>> {
        todo!()
    }

    fn get_parent(&mut self, node_id: NodeId<Key, Value>) -> Option<&Value> {
        todo!()
    }

    fn get_parent_mut(&mut self, node_id: NodeId<Key, Value>) -> Option<&mut Value> {
        todo!()
    }

    fn get_left_uncle_id(&self, node_id: NodeId<Key, Value>) -> Option<NodeId<Key, Value>> {
        todo!()
    }

    fn get_right_uncle_id(&self, node_id: NodeId<Key, Value>) -> Option<NodeId<Key, Value>> {
        todo!()
    }

    fn get_left_uncle_mut(&mut self, node_id: NodeId<Key, Value>) -> Option<&mut Value> {
        todo!()
    }

    fn get_right_uncle_mut(&mut self, node_id: NodeId<Key, Value>) -> Option<&mut Value> {
        todo!()
    }

    fn is_left_son(&self, node_id: NodeId<Key, Value>) -> bool {
        todo!()
    }

    fn is_right_son(&self, node_id: NodeId<Key, Value>) -> bool {
        todo!()
    }
}

impl<Key, Value> RotatableTree<Key, Value, NodeId<Key, Value>> for ListBST<Key, Value>
where
    Key: Ord,
    Value: Clone,
{
    fn right_rotate(&mut self, node_id: NodeId<Key, Value>) {
        todo!()
    }

    fn left_rotate(&mut self, node_id: NodeId<Key, Value>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::list_bst::ListBST;
    use crate::traits::{NodeIdentifiableTree, ParentifiedTree, RotatableTree, Tree};

    #[test]
    fn test_binary_tree() {
        let mut tree = ListBST::<i32, i32>::default();

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
            assert_eq!(found.unwrap(), val);
        }

        for non_existent in 1000..2000 {
            let found = tree.get(&non_existent);
            assert!(found.is_none());
        }
    }
}
