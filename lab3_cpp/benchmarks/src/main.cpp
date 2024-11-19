#include "../include/bst_benchmarks.hpp"

BENCHMARK(BM_array_bst_insertion);
BENCHMARK(BM_linked_bst_insertion);
BENCHMARK(BM_array_bst_search);
BENCHMARK(BM_linked_bst_search);

BENCHMARK_MAIN();
