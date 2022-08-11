cmake_minimum_required(VERSION 3.8)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
project(${puzzle} CXX)
set(CMAKE_CXX_STANDARD 11)
add_executable(puzzle puzzle.cpp)

