#include <stdexcept>
#include "../../red_black_trees/include/LinkedRedBlackTree.hpp"
#include "../include/rb_benchmarks.hpp"

void BM_linked_rb_insertion(benchmark::State& state)
{
    LinkedRedBlackTree<int, int> tree;
    auto next_number = [] {
        static int number = 0;
        return number++;
    };

    for (auto _ : state)
    {
        tree.insert(next_number(), next_number() + 1);
    }
}

void BM_linked_rb_search(benchmark::State& state)
{
    LinkedRedBlackTree<int, int> tree;

    for (int i = 0; i < BIG_RB_TREE_SIZE; ++i)
    {
        tree.insert(+i, i + 1);
    }

    int found_value = 0;

    for (auto _ : state)
    {
        found_value = tree.get(BIG_RB_TREE_SIZE - 1);
    }

    if (found_value != BIG_RB_TREE_SIZE)
    {
        throw std::runtime_error("Wrong value found");
    }
}
