#include <cassert>
#include <iostream>

#include "../include/ArrayBST.hpp"

int main()
{
    ArrayBST<int, int> tree;
    tree.insert(1, 13);
    tree.insert(2, 14);
    tree.insert(3, 15);
    tree.insert(-1, 16);

    assert(tree.get(1) == 13);
    assert(tree.get(2) == 14);
    assert(tree.get(3) == 15);
    assert(tree.get(-1) == 16);

    std::cout << tree.get(1) << std::endl;
    std::cout << tree.get(2) << std::endl;
    std::cout << tree.get(3) << std::endl;
    std::cout << tree.get(-1) << std::endl;

    std::cout << "Root: " << tree.get_root() << std::endl;
    std::cout << "Right Son of Root: " << tree.get_right_son(*tree.get_root_id()) << std::endl;
    std::cout << "Left Son of Root: " << tree.get_left_son(*tree.get_root_id()) << std::endl;
    std::cout << "Right Son of Right Son of Root: " <<
        tree.get_right_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    std::cout << "Left uncle of the Right Son of Right Son of Root: " <<
        tree.get_left_uncle(*tree.get_right_son_id(*tree.get_right_son_id(*tree.get_root_id()))) << std::endl;

    std::cout << "Is right son of root a left son: " << tree.is_left_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    std::cout << "Is left son of root a left son: " << tree.is_left_son(*tree.get_left_son_id(*tree.get_root_id())) << std::endl;
    std::cout << "Is right son of root a right son: " << tree.is_right_son(*tree.get_right_son_id(*tree.get_root_id())) << std::endl;
    std::cout << "Is left son of root a right son: " << tree.is_right_son(*tree.get_left_son_id(*tree.get_root_id())) << std::endl;
}
