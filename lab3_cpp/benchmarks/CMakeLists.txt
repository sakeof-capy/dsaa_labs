add_executable(rb_bench src/main.cpp
        ../red_black_trees/include/ArrayBST.hpp
        ../red_black_trees/src/main.cpp
)
target_link_libraries(rb_bench benchmark::benchmark)
