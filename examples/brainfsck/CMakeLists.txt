cmake_minimum_required(VERSION 3.1.0 FATAL_ERROR)

project(brainfsck)

add_executable(brainfsck brainfsck.cpp)

add_definitions("-Wall -Werror")

set_property(TARGET brainfsck PROPERTY CXX_STANDARD 11)
set_property(TARGET brainfsck PROPERTY CXX_STANDARD_REQUIRED ON)
