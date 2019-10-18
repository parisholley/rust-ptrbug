# A skeleton CMake Build file
# Builds some basic C examples
cmake_minimum_required(VERSION 3.4)
project(ptrbug)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(BUG lib.c
        lib.h)
