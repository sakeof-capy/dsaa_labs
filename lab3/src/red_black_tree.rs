use crate::traits::{NodeIdentifiableTree, ParentifiedTree, RotatableTree, Tree};
use std::marker::PhantomData;

type Color = bool;

const RED: Color = true;
const BLACK: Color = false;

pub(super) struct Data<Value> {
    value: Value,
    color: Color,
}

impl<Value> std::fmt::Display for Data<Value>
where
    Value: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{} | {}]",
            self.value,
            if self.color == RED { "RED" } else { "BLACK" }
        )
    }
}

pub struct RedBlackTree<Key, Value, ImplTree, NodeId>
where
    Key: Ord,
    NodeId: Default + Clone + Copy + PartialEq + Eq,
    ImplTree: Default
        + Tree<Key, Data<Value>>
        + NodeIdentifiableTree<Key, Data<Value>, NodeId>
        + ParentifiedTree<Key, Data<Value>, NodeId>
        + RotatableTree<Key, Data<Value>, NodeId>,
{
    bst: ImplTree,
    _phantom_key: PhantomData<Key>,
    _phantom_value: PhantomData<Value>,
    _phantom_node_id: PhantomData<NodeId>,
}

impl<Key, Value, ImplTree, NodeId> Default for RedBlackTree<Key, Value, ImplTree, NodeId>
where
    Key: Ord,
    NodeId: Default + Clone + Copy + PartialEq + Eq,
    ImplTree: Default
        + Tree<Key, Data<Value>>
        + NodeIdentifiableTree<Key, Data<Value>, NodeId>
        + ParentifiedTree<Key, Data<Value>, NodeId>
        + RotatableTree<Key, Data<Value>, NodeId>,
{
    fn default() -> Self {
        Self {
            bst: Default::default(),
            _phantom_key: Default::default(),
            _phantom_value: Default::default(),
            _phantom_node_id: Default::default(),
        }
    }
}

impl<Key, Value, ImplTree, NodeId> Tree<Key, Value> for RedBlackTree<Key, Value, ImplTree, NodeId>
where
    Key: Ord,
    NodeId: Default + Clone + Copy + PartialEq + Eq,
    ImplTree: Default
        + Tree<Key, Data<Value>>
        + NodeIdentifiableTree<Key, Data<Value>, NodeId>
        + ParentifiedTree<Key, Data<Value>, NodeId>
        + RotatableTree<Key, Data<Value>, NodeId>,
{
    fn insert(&mut self, key: Key, val: Value) {
        let mut z_node_id = self.bst.insert_and_get_id(
            key,
            Data {
                value: val,
                color: RED,
            },
        );

        while let Some(z_node) = self.bst.get_by_id(z_node_id)
            && z_node.color == RED
            && self.bst.get_root_id() != z_node_id
        {
            let z_parent_id = self.bst.get_parent_id(z_node_id).unwrap();

            if self.bst.is_left_son(z_parent_id) {
                if let Some(z_right_uncle_id) = self.bst.get_right_uncle_id(z_node_id)
                    && let Some(z_right_uncle) = self.bst.get_by_id(z_right_uncle_id)
                    && z_right_uncle.color == RED
                {
                    if let Some(z_grand_parent_id) = self.bst.get_parent_id(z_parent_id) {
                        self.bst.modify(z_parent_id, |val| val.color = BLACK);
                        self.bst.modify(z_right_uncle_id, |val| val.color = BLACK);
                        self.bst.modify(z_grand_parent_id, |val| val.color = RED);
                        z_node_id = z_grand_parent_id;
                    }
                } else if self.bst.is_right_son(z_node_id) {
                    z_node_id = z_parent_id;
                    self.bst.left_rotate(z_parent_id);
                } else if let Some(z_grand_parent_id) = self.bst.get_parent_id(z_parent_id) {
                    z_node_id = z_parent_id;
                    self.bst.modify(z_parent_id, |val| val.color = BLACK);
                    self.bst.modify(z_grand_parent_id, |val| val.color = RED);
                    self.bst.right_rotate(z_grand_parent_id);
                }
            } else if self.bst.is_right_son(z_parent_id) {
            } else {
                break;
            }
        }

        let root = self.bst.get_root_mut().unwrap();
        root.color = BLACK;

        // while let Some(z_node) = self.bst.get_by_id(z_node_id)
        //     && z_node.color == RED
        // {
        //     if let Some(z_parent_id) = self.bst.get_parent_id(z_node_id)
        //         && let Some(z_left_uncle_id) = self.bst.get_left_uncle_id(z_node_id)
        //         && let Some(z_right_uncle_id) = self.bst.get_right_uncle_id(z_node_id)
        //         && let Some(z_parent) = self.bst.get_by_id_mut(z_parent_id)
        //         && let Some(z_grandparent_id) = self.bst.get_parent_id(z_parent_id)
        //         && let Some(z_grandparent) = self.bst.get_by_id_mut(z_grandparent_id)
        //         && let Some(z_left_uncle) = self.bst.get_by_id(z_left_uncle_id)
        //         && let Some(z_right_uncle) = self.bst.get_by_id_mut(z_right_uncle_id)
        //         && z_parent_id == z_left_uncle_id
        //     {
        //         if z_right_uncle.color == RED {
        //             z_parent.color = BLACK;
        //             z_right_uncle.color = BLACK;
        //             z_grandparent.color = RED;
        //             z_node_id = z_grandparent_id;
        //         } else if let Some(right_brother_id) = self.bst.get_right_son_id(z_parent_id)
        //             && z_node_id == right_brother_id
        //         {
        //             z_node_id = z_parent_id;
        //             self.bst.left_rotate(z_parent_id);
        //         } else {
        //             z_parent.color = BLACK;
        //             z_grandparent.color = RED;
        //             self.bst.right_rotate(z_grandparent_id);
        //         }
        //     }
        // }
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        self.bst.get(key).map(|x| &x.value)
    }

    fn delete(&mut self, key: &Key) -> Option<Value> {
        todo!()
    }
}

impl<Key, Value, ImplTree, NodeId> std::fmt::Display for RedBlackTree<Key, Value, ImplTree, NodeId>
where
    Key: Ord + std::fmt::Display,
    Value: std::fmt::Display,
    NodeId: Default + Clone + Copy + PartialEq + Eq,
    ImplTree: Default
        + Tree<Key, Data<Value>>
        + NodeIdentifiableTree<Key, Data<Value>, NodeId>
        + ParentifiedTree<Key, Data<Value>, NodeId>
        + RotatableTree<Key, Data<Value>, NodeId>
        + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.bst)
    }
}
