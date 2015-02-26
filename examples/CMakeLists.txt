cmake_minimum_required(VERSION 3.1.0 FATAL_ERROR)
project(senior_project_examples)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/Modules/RustCMake/cmake")
list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/Modules/cmake-d/cmake-d")

add_subdirectory(hello-world)
add_subdirectory(sentence-splitter)
add_subdirectory(int-linked-list)
add_subdirectory(generic-array-list)
add_subdirectory(brainfsck)
