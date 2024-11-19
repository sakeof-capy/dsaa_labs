#ifndef BENCHMARKS_HPP
#define BENCHMARKS_HPP
#include <benchmark/benchmark.h>

void BM_array_tree_insertion(benchmark::State& state);
void BM_list_tree_insertion(benchmark::State& state);

void BM_array_tree_search(benchmark::State& state);
void BM_list_tree_search(benchmark::State& state);

void BM_array_tree_deletion(benchmark::State& state);
void BM_list_tree_deletion(benchmark::State& state);

#endif //BENCHMARKS_HPP
