add_executable(bst_bench
    src/main.cpp
    src/array.cpp
    src/list.cpp
)

add_executable(rb_bench
    src/rb_main.cpp
    src/rb_array.cpp
    src/rb_linked.cpp
    src/rb_std_map.cpp
)

target_link_libraries(bst_bench benchmark::benchmark)
target_link_libraries(rb_bench benchmark::benchmark)
