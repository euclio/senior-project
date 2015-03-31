cmake_minimum_required(VERSION 3.1.0 FATAL_ERROR)
project(sentence_splitter)

add_executable(sentence_splitter sentence_splitter.cpp)

add_definitions("-Wall -Werror")

set_property(TARGET sentence_splitter PROPERTY CXX_STANDARD 11)
set_property(TARGET sentence_splitter PROPERTY CXX_STANDARD_REQUIRED ON)
