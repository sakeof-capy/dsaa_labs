#include "../include/rb_benchmarks.hpp"

BENCHMARK(BM_array_rb_insertion);
BENCHMARK(BM_linked_rb_insertion);
BENCHMARK(BM_std_map_insertion);
BENCHMARK(BM_array_rb_search);
BENCHMARK(BM_linked_rb_search);
BENCHMARK(BM_std_map_search);

BENCHMARK_MAIN();
