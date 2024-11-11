#![feature(let_chains)]

use crate::array_bst::ArrayBST;
// use crate::array_binary_search_tree::ArrayBinarySearchTree;
// use crate::red_black_tree::RedBlackTree;
use crate::traits::Tree;

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
    let mut tree = ArrayBST::default();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(-1, -1);

    println!("{}", tree);
}