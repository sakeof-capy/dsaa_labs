#![feature(let_chains)]

use crate::array_bst::ArrayBST;
// use crate::array_binary_search_tree::ArrayBinarySearchTree;
// use crate::red_black_tree::RedBlackTree;
use crate::traits::{NodeIdentifiableTree, RotatableTree, Tree};

// mod array_binary_search_tree;
mod array_bst;
// mod red_black_tree;
mod traits;

// pub type ArrayRedBlackTree<Key, Value> = RedBlackTree<
//     Key,
//     Value,
//     ArrayBinarySearchTree<Key, red_black_tree::Data<Value>>,
//     array_binary_search_tree::NodeId,
// >;
//
// fn main() {
//     let mut tree = ArrayRedBlackTree::default();
//     tree.insert(15, 0);
//     println!("{}", tree);
//     tree.insert(5, 0);
//     println!("{}", tree);
//     tree.insert(1, 0);
//     println!("{}", tree);
// }

fn main() {
    {
        let mut tree = ArrayBST::default();
        tree.insert(1000, 0);
        let id = tree.insert_and_get_id(5, 0);
        tree.insert(4, 0);
        tree.insert(7, 0);
        tree.insert(6, 0);
        tree.insert(8, 0);

        println!("{}", tree);

        tree.left_rotate(id);
        println!("Left rotated:\n {}", tree);
    }
    {
        let mut tree = ArrayBST::default();
        tree.insert(1000, 0);
        let id = tree.insert_and_get_id(5, 0);
        tree.insert(3, 0);
        tree.insert(1, 0);
        tree.insert(4, 0);
        tree.insert(6, 0);

        println!("{}", tree);

        tree.right_rotate(id);
        println!("Left rotated:\n {}", tree);
    }
}
