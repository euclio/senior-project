cmake_minimum_required(VERSION 3.1.0 FATAL_ERROR)
project(int-linked-list)

add_executable(int-linked-list int_linked_list.cpp int_linked_list.hpp)

add_definitions("-Wall -Werror")

set_property(TARGET int-linked-list PROPERTY CXX_STANDARD 11)
set_property(TARGET int-linked-list PROPERTY CXX_STANDARD_REQUIRED ON)
