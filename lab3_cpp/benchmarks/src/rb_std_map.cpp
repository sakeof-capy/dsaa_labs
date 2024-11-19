#include <map>
#include <stdexcept>
#include "../include/rb_benchmarks.hpp"

void BM_std_map_insertion(benchmark::State& state)
{
    std::map<int, int> tree;
    auto next_number = [] {
        static int number = 0;
        return number++;
    };

    for (auto _ : state)
    {
        tree[next_number()] = next_number() + 1;
    }
}

void BM_std_map_search(benchmark::State& state)
{
    std::map<int, int> tree;

    for (int i = 0; i < BIG_RB_TREE_SIZE; ++i)
    {
        tree[i] = i + 1;
    }

    int found_value = 0;

    for (auto _ : state)
    {
        found_value = tree[BIG_RB_TREE_SIZE - 1];
    }

    if (found_value != BIG_RB_TREE_SIZE)
    {
        throw std::runtime_error("Wrong value found");
    }
}

void BM_std_map_deletion(benchmark::State& state);
