#include "../../red_black_trees/include/ListBST.hpp"
#include "../include/benchmarks.hpp"

void BM_list_tree_insertion(benchmark::State& state)
{
    LinkedBST<int, int> tree;
    auto next_number = [] {
        static int number = 0;
        return number++;
    };

    for (auto _ : state)
    {
        tree.insert(next_number(), next_number() + 1);
    }
}

void BM_list_tree_search(benchmark::State& state);
void BM_list_tree_deletion(benchmark::State& state);
