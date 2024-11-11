pub trait Tree<Key, Value> {
    fn insert(&mut self, key: Key, val: Value);
    fn get(&self, key: &Key) -> Option<&Value>;
    fn delete(&mut self, key: &Key) -> Option<Value>;
}

pub trait NodeIdentifiableTree<Key, Value, NodeId> {
    fn insert_and_get_id(&mut self, key: Key, val: Value) -> NodeId;

    fn get_by_id(&self, node_id: NodeId) -> Option<&Value>;
    fn get_by_id_mut(&mut self, node_id: NodeId) -> Option<&mut Value>;

    fn get_left_son_id(&self, node_id: NodeId) -> Option<NodeId>;
    fn get_right_son_id(&self, node_id: NodeId) -> Option<NodeId>;
    fn get_root_id(&self) -> NodeId;

    fn get_left_son_mut(&mut self, node_id: NodeId) -> Option<&mut Value>;
    fn get_right_son_mut(&mut self, node_id: NodeId) -> Option<&mut Value>;
    fn get_root_mut(&mut self) -> Option<&mut Value>;

    fn modify<F>(&mut self, node_id: NodeId, modifier: F)
    where
        F: FnMut(&mut Value);
}

pub trait ParentifiedTree<Key, Value, NodeId>: NodeIdentifiableTree<Key, Value, NodeId> {
    fn get_parent_id(&self, node_id: NodeId) -> Option<NodeId>;
    fn get_parent(&mut self, node_id: NodeId) -> Option<&Value>;
    fn get_parent_mut(&mut self, node_id: NodeId) -> Option<&mut Value>;
    fn get_left_uncle_id(&self, node_id: NodeId) -> Option<NodeId>;
    fn get_right_uncle_id(&self, node_id: NodeId) -> Option<NodeId>;
    fn get_left_uncle_mut(&mut self, node_id: NodeId) -> Option<&mut Value>;
    fn get_right_uncle_mut(&mut self, node_id: NodeId) -> Option<&mut Value>;
    fn is_left_son(&self, node_id: NodeId) -> bool;
    fn is_right_son(&self, node_id: NodeId) -> bool;
}

pub trait RotatableTree<Key, Value, NodeId>: NodeIdentifiableTree<Key, Value, NodeId> {
    fn right_rotate(&mut self, node_id: NodeId);
    fn left_rotate(&mut self, node_id: NodeId);
}
