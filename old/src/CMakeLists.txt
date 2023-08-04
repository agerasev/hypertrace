cmake_minimum_required(VERSION 3.0)
project(hypertrace)


set(CMAKE_C_FLAGS "-DHOST -Wall")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++17")

include("Toolchains.cmake")

add_subdirectory(common)
add_subdirectory(device)
add_subdirectory(host)
add_subdirectory(examples)
add_subdirectory(test)
