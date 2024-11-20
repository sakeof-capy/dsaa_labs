#include <iostream>
// #include "../include/ArrayRedBlackTree.hpp"
#include "../include/LinkedRedBlackTree.hpp"

void test_linked()
{
    // // Left rotation
    // ArrayBST<int, int> tree;
    // NodeId root_id = tree.insert_and_get_id(5, 13);
    // tree.insert(4, 14);
    // tree.insert(7, 15);
    // tree.insert(8, 16);
    // tree.insert(6, 17);
    //
    // std::cout << "The tree:\n" << tree << std::endl << std::endl;
    //
    // tree.left_rotate(root_id);
    //
    // std::cout << "Rotated tree:\n" << tree << std::endl << std::endl;

    // // Right rotation
    // ArrayBST<int, int> tree;
    // NodeId root_id = tree.insert_and_get_id(5, 13);
    // tree.insert(3, 15);
    // tree.insert(6, 16);
    // tree.insert(1, 17);
    // tree.insert(4, 18);
    //
    // std::cout << "The tree:\n" << tree << std::endl << std::endl;
    //
    // tree.right_rotate(root_id);
    //
    // std::cout << "Rotated tree:\n" << tree << std::endl << std::endl;

    // LinkedBST<int, int> tree;
    //
    // NodeId root = tree.insert_and_get_id(1, 13);
    // tree.insert(2, 14);
    // tree.insert(3, 15);
    // tree.insert(-1, 16);
    //
    // assert(tree.get(1) == 13);
    // assert(tree.get(2) == 14);
    // assert(tree.get(3) == 15);
    // assert(tree.get(-1) == 16);

    // std::cout << "The tree:\n" << tree << std::endl << std::endl;

    // std::cout << tree.get(1) << std::endl;
    // std::cout << tree.get(2) << std::endl;
    // std::cout << tree.get(3) << std::endl;
    // std::cout << tree.get(-1) << std::endl;
    //
    // std::cout << "Root: " << tree.get_root() << std::endl;
    //
    // std::cout << "Right Son of Root: " << tree.get_right_son(*tree.get_root_id()) << std::endl;
    // std::cout << "Left Son of Root: " << tree.get_left_son(*tree.get_root_id()) << std::endl;
    // std::cout << "Right Son of Right Son of Root: " <<
    //     tree.get_right_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    // std::cout << "Left uncle of the Right Son of Right Son of Root: " <<
    //     tree.get_left_uncle(*tree.get_right_son_id(*tree.get_right_son_id(*tree.get_root_id()))) << std::endl;
    //
    // std::cout << "Is right son of root a left son: " << tree.is_left_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    // std::cout << "Is left son of root a left son: " << tree.is_left_son(*tree.get_left_son_id(*tree.get_root_id())) << std::endl;
    // std::cout << "Is right son of root a right son: " << tree.is_right_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    // std::cout << "Is left son of root a right son: " << tree.is_right_son(*tree.get_left_son_id(*tree.get_root_id())) << std::endl;

    // // Left rotation:
    // LinkedBST<int, int> tree;
    // const NodeId root_id = tree.insert_and_get_id(5, 13);
    // tree.insert(4, 14);
    // tree.insert(7, 15);
    // tree.insert(8, 16);
    // tree.insert(6, 17);
    //
    // std::cout << "The tree:\n" << tree << std::endl << std::endl;
    //
    // tree.left_rotate(root_id);
    //
    // std::cout << "Rotated tree:\n" << tree << std::endl << std::endl;

    // // Right Rotation
    // LinkedBST<int, int> tree;
    // const NodeId root_id = tree.insert_and_get_id(5, 13);
    // tree.insert(3, 15);
    // tree.insert(6, 16);
    // tree.insert(1, 17);
    // tree.insert(4, 18);
    //
    // std::cout << "The tree:\n" << tree << std::endl << std::endl;
    //
    // tree.right_rotate(root_id);
    //
    // std::cout << "Rotated tree:\n" << tree << std::endl << std::endl;

}

int main()
{
    test_linked();
    // // Red-Black Tree
    LinkedRedBlackTree<int, int> rbtree;
    rbtree.insert(8, 13);
    rbtree.insert(5, 13);
    rbtree.insert(15, 13);
    rbtree.insert(12, 13);
    rbtree.insert(19, 13);
    rbtree.insert(9, 13);
    rbtree.insert(13, 13);
    rbtree.insert(23, 13);
    rbtree.insert(10, 13);

    std::cout << "RBTree:\n" << rbtree << std::endl;

    // // Left rotation
    // ArrayBST<int, int> tree;
    // NodeId root_id = tree.insert_and_get_id(5, 13);
    // tree.insert(4, 14);
    // tree.insert(7, 15);
    // tree.insert(8, 16);
    // tree.insert(6, 17);
    //
    // std::cout << "The tree:\n" << tree << std::endl << std::endl;
    //
    // tree.left_rotate(root_id);
    //
    // std::cout << "Rotated tree:\n" << tree << std::endl << std::endl;

    // // Right rotation
    // ArrayBST<int, int> tree;
    // NodeId root_id = tree.insert_and_get_id(5, 13);
    // tree.insert(3, 15);
    // tree.insert(6, 16);
    // tree.insert(1, 17);
    // tree.insert(4, 18);
    //
    // std::cout << "The tree:\n" << tree << std::endl << std::endl;
    //
    // tree.right_rotate(root_id);
    //
    // std::cout << "Rotated tree:\n" << tree << std::endl << std::endl;

    // ArrayBST<int, int> tree;
    // tree.insert(1, 13);
    // tree.insert(2, 14);
    // tree.insert(3, 15);
    // tree.insert(-1, 16);
    //
    // assert(tree.get(1) == 13);
    // assert(tree.get(2) == 14);
    // assert(tree.get(3) == 15);
    // assert(tree.get(-1) == 16);
    //
    // std::cout << "The tree:\n" << tree << std::endl << std::endl;
    //
    // std::cout << tree.get(1) << std::endl;
    // std::cout << tree.get(2) << std::endl;
    // std::cout << tree.get(3) << std::endl;
    // std::cout << tree.get(-1) << std::endl;
    //
    // std::cout << "Root: " << tree.get_root() << std::endl;
    // std::cout << "Right Son of Root: " << tree.get_right_son(*tree.get_root_id()) << std::endl;
    // std::cout << "Left Son of Root: " << tree.get_left_son(*tree.get_root_id()) << std::endl;
    // std::cout << "Right Son of Right Son of Root: " <<
    //     tree.get_right_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    // std::cout << "Left uncle of the Right Son of Right Son of Root: " <<
    //     tree.get_left_uncle(*tree.get_right_son_id(*tree.get_right_son_id(*tree.get_root_id()))) << std::endl;
    //
    // std::cout << "Is right son of root a left son: " << tree.is_left_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    // std::cout << "Is left son of root a left son: " << tree.is_left_son(*tree.get_left_son_id(*tree.get_root_id())) << std::endl;
    // std::cout << "Is right son of root a right son: " << tree.is_right_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    // std::cout << "Is left son of root a right son: " << tree.is_right_son(*tree.get_left_son_id(*tree.get_root_id())) << std::endl;
}
