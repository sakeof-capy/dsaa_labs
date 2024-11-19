#ifndef BENCHMARKS_HPP
#define BENCHMARKS_HPP
#include <benchmark/benchmark.h>

constexpr std::size_t BIG_TREE_SIZE = 10000;

void BM_array_bst_insertion(benchmark::State& state);
void BM_linked_bst_insertion(benchmark::State& state);

void BM_array_bst_search(benchmark::State& state);
void BM_linked_bst_search(benchmark::State& state);

void BM_array_bst_deletion(benchmark::State& state);
void BM_linked_bst_deletion(benchmark::State& state);

#endif //BENCHMARKS_HPP
