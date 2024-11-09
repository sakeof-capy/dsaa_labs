#![feature(let_chains)]

use crate::array_binary_search_tree::ArrayBinarySearchTree;
use crate::traits::Tree;

mod array_binary_search_tree;
mod red_black_tree;
mod traits;

fn main() {
    let mut tree = ArrayBinarySearchTree::default();
    tree.insert(1, "a");
    tree.insert(2, "b");
    tree.insert(3, "c");
    tree.insert(-1, "e");
    tree.insert(0, "f");
    println!("{}", tree);
}
