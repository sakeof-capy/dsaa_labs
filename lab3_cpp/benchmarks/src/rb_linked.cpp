#include <stdexcept>
#include "../include/rb_benchmarks.hpp"

void BM_array_rb_insertion(benchmark::State& state);
void BM_linked_rb_insertion(benchmark::State& state);
void BM_std_map_insertion(benchmark::State& state);

void BM_array_rb_search(benchmark::State& state);
void BM_linked_rb_search(benchmark::State& state);
void BM_std_map_search(benchmark::State& state);

void BM_array_rb_deletion(benchmark::State& state);
void BM_linked_rb_deletion(benchmark::State& state);
void BM_std_map_deletion(benchmark::State& state);
