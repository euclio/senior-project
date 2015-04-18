cmake_minimum_required(VERSION 3.1.0 FATAL_ERROR)
project(parallel-merge-sort)

find_package(Threads)

add_executable(parallel-merge-sort parallel_merge_sort.cpp)

add_definitions("-Wall -Werror")

target_link_libraries(parallel-merge-sort ${CMAKE_THREAD_LIBS_INIT})

set_property(TARGET parallel-merge-sort PROPERTY CXX_STANDARD 11)
set_property(TARGET parallel-merge-sort PROPERTY CXX_STANDARD_REQUIRED ON)
