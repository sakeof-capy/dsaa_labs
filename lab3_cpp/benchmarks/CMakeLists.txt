add_executable(rb_bench
    src/main.cpp
    src/array.cpp
    src/list.cpp
        include/benchmarks.hpp
)

target_link_libraries(rb_bench benchmark::benchmark)
