cmake_minimum_required(VERSION 3.5)
project(red_black_trees)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(BENCHMARK_DOWNLOAD_DEPENDENCIES ON)

add_subdirectory(dependencies/benchmark)
add_subdirectory(red_black_trees)
add_subdirectory(benchmarks)
