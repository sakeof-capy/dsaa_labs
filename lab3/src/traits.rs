pub trait Tree<Key, Value> {
    fn insert(&mut self, key: Key, val: Value);
    fn get(&self, key: &Key) -> Option<&Value>;
    fn delete(&mut self, key: &Key) -> Option<Value>;
}
