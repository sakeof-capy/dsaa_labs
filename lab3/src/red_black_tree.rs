use std::marker::PhantomData;
use crate::array_binary_search_tree::ArrayBinarySearchTree;
use crate::traits::Tree;

type Color = bool;

const RED: Color = true;
const BLACK: Color = false;

struct Data<Value> {
    value: Value,
    color: Color,
}

struct RedBlackTree<Key, Value, ImplTree>
where
    Key: Ord,
    ImplTree: Default + Tree<Key, Data<Value>>,
{
    binary_search_tree: ImplTree,
    _phantom_key: PhantomData<Key>,
    _phantom_value: PhantomData<Value>,
}

impl<Key, Value, ImplTree> Tree<Key, Value> for RedBlackTree<Key, Value, ImplTree>
where
    Key: Ord,
    ImplTree: Default + Tree<Key, Data<Value>>,
{
    fn insert(&mut self, key: Key, val: Value) {
        self.binary_search_tree.insert(key, Data { value: val, color: RED });
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        todo!()
    }

    fn delete(&mut self, key: &Key) -> Option<Value> {
        todo!()
    }
}

impl<Key, Value, ImplTree> std::fmt::Display for RedBlackTree<Key, Value, ImplTree>
where
    Key: Ord + std::fmt::Display,
    Value: std::fmt::Display,
    ImplTree: Default + Tree<Key, Data<Value>> + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.binary_search_tree)
    }
}
