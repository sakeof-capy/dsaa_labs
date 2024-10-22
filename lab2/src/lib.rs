use crate::pearson_map::{Pair, PearsonMap};
use common::subcontainers::array_based_deque::*;

pub mod pearson_map;

pub type PersonArrayMap<ValueType> = PearsonMap<ValueType, ArrayBasedDeque<Pair<ValueType>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_hashmap_test() {
        let mut hash_map = PersonArrayMap::<i32>::default();

        assert_eq!(hash_map.size(), 0usize);
        assert_eq!(hash_map.find("a"), None);
        assert_eq!(hash_map.get("a"), None);

        hash_map.put("a".to_owned(), 1);
        hash_map.put("b".to_owned(), 2);
        hash_map.put("c".to_owned(), 3);
        hash_map.put("d".to_owned(), 4);

        let unwrap_option = |option: Option<&i32>| {
            assert!(option.is_some());
            *option.unwrap()
        };

        assert_eq!(unwrap_option(hash_map.find("a")), 1);
        assert_eq!(unwrap_option(hash_map.find("b")), 2);
        assert_eq!(unwrap_option(hash_map.find("c")), 3);
        assert_eq!(unwrap_option(hash_map.find("d")), 4);

        hash_map.put("a".to_owned(), 5);
        assert_eq!(unwrap_option(hash_map.find("a")), 5);

        for c in 'a'..'z' {
            hash_map.put(c.to_string(), c as i32);
        }

        for c in 'a'..'z' {
            assert_eq!(unwrap_option(hash_map.find(&c.to_string())), c as i32);
        }

        for i in 0..128 {
            hash_map.put(format!("string{}", i), i);
        }

        for i in 0..128 {
            assert_eq!(unwrap_option(hash_map.find(&format!("string{}", i))), i);
        }
    }
}
