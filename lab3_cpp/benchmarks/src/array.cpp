#include "../../red_black_trees/include/ArrayBST.hpp"
#include "../include/benchmarks.hpp"

void BM_array_tree_insertion(benchmark::State& state)
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

void BM_array_tree_search(benchmark::State& state);
void BM_array_tree_deletion(benchmark::State& state);
