#include "../../red_black_trees/include/ArrayBST.hpp"
#include "../include/bst_benchmarks.hpp"

void BM_array_bst_insertion(benchmark::State& state)
{
    ArrayBST<int, int> tree;
    auto next_number = [] {
        static int number = 0;
        return number++;
    };

    for (auto _ : state)
    {
        tree.insert(next_number(), next_number() + 1);
    }
}

void BM_array_bst_search(benchmark::State& state)
{
    ArrayBST<int, int> tree;

    for (int i = 0; i < BIG_TREE_SIZE; ++i)
    {
        tree.insert(+i, i + 1);
    }

    int found_value = 0;

    for (auto _ : state)
    {
        found_value = tree.get(BIG_TREE_SIZE - 1);
    }

    if (found_value != BIG_TREE_SIZE)
    {
        throw std::runtime_error("Wrong value found");
    }
}

void BM_array_bst_deletion(benchmark::State& state);
