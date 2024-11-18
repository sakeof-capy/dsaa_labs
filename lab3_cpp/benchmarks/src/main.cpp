#include <benchmark/benchmark.h>

static void BM_SmallStringCopy(benchmark::State& state) {
    std::string x = "12345678901234";
    for (auto _ : state)
        std::string copy(x);
}
// Register the function as a benchmark
BENCHMARK(BM_SmallStringCopy);

// Define another benchmark
static void BM_BigStringCopy(benchmark::State& state) {
  std::string x = "123456789012345";
  for (auto _ : state)
    std::string copy(x);
}
BENCHMARK(BM_BigStringCopy);

BENCHMARK_MAIN();
