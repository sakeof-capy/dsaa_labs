#![feature(let_chains)]

use crate::array_bst::ArrayBST;
use crate::red_black_tree::RedBlackTree;
use crate::traits::Tree;

mod array_bst;
mod red_black_tree;
mod traits;
mod list_bst;

pub type ArrayRedBlackTree<'l, Key, Value> = RedBlackTree<
    Key,
    Value,
    ArrayBST<Key, red_black_tree::Data<Value>>,
    array_bst::NodeId,
>;

fn main() {
    let mut tree = ArrayRedBlackTree::<i32, i32>::default();
    tree.insert(15, 0);
    tree.insert(5, 1);
    tree.insert(1, 2);

    println!("{}", tree);

    // {
    //     let mut tree = ArrayBST::default();
    //     tree.insert(1000, 0);
    //     let id = tree.insert_and_get_id(5, 0);
    //     tree.insert(4, 0);
    //     tree.insert(7, 0);
    //     tree.insert(6, 0);
    //     tree.insert(8, 0);
    //
    //     println!("{}", tree);
    //
    //     tree.left_rotate(id);
    //     println!("Left rotated:\n {}", tree);
    // }
    // {
    //     let mut tree = ArrayBST::default();
    //     tree.insert(1000, 0);
    //     let id = tree.insert_and_get_id(5, 0);
    //     tree.insert(3, 0);
    //     tree.insert(1, 0);
    //     tree.insert(4, 0);
    //     tree.insert(6, 0);
    //
    //     println!("{}", tree);
    //
    //     tree.right_rotate(id);
    //     println!("Left rotated:\n {}", tree);
    // }
}
