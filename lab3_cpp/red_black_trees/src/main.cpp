#include "../include/ArrayBST.hpp"
#include <cassert>
#include <iostream>

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
}
