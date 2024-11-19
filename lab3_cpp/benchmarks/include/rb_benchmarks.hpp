#ifndef RB_BENCHMARKS_HPP
#define RB_BENCHMARKS_HPP
#include <benchmark/benchmark.h>

constexpr std::size_t BIG_RB_TREE_SIZE = 10000;

void BM_array_rb_insertion(benchmark::State& state);
void BM_linked_rb_insertion(benchmark::State& state);
void BM_std_map_insertion(benchmark::State& state);

void BM_array_rb_search(benchmark::State& state);
void BM_linked_rb_search(benchmark::State& state);
void BM_std_map_search(benchmark::State& state);

void BM_array_rb_deletion(benchmark::State& state);
void BM_linked_rb_deletion(benchmark::State& state);
void BM_std_map_deletion(benchmark::State& state);

#endif //RB_BENCHMARKS_HPP
