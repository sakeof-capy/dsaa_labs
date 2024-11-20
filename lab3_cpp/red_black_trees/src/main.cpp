#include <iostream>
#include "../include/ArrayRedBlackTree.hpp"

int main() {
    int N;

    std::cin >> N;

    if (N <= 1 || N >= 256) {
        std::cerr << "N must be in the range 1 < N < 256!" << std::endl;
        return EXIT_FAILURE;
    }

    ArrayRedBlackTree<int, int> tree;
    std::cout << "Initial tree:\n" << tree << std::endl;

    for (int i = 0; i < N; ++i) {
        int key, val;
        std::cin >> key >> val;
        std::cout << "Inserting key = " << key << " value = " << val << std::endl;
        tree.insert(+key, +val);
        auto min_elem_opt = tree.min_elem();

        if (min_elem_opt.has_value())
        {
            auto [min_key, min_val, min_clr] = min_elem_opt.value();
            std::cout << "current minimal element = (" << min_key << ", " << min_val << ") of color " << min_clr <<std::endl;
        }
        else
        {
            std::cout << "there's no minimal element currently" <<std::endl;
        }

        std::cout << "current tree:\n" << tree << std::endl;
    }

    return EXIT_SUCCESS;
}