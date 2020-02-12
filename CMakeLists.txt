cmake_minimum_required(VERSION 2.6)
project(hypertrace)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -std=c++14 -Wall -Wextra")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -Wall -Wextra")

set(SRC)
include_directories(src)


# Main target

add_executable(${PROJECT_NAME} ${SRC} src/host/main.cpp)
target_link_libraries(${PROJECT_NAME} OpenCL SDL2)


# Test target

add_executable(test ${SRC} src/host/test.cpp)
target_compile_definitions(test PRIVATE -DTEST)

if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    target_compile_options(test PRIVATE --coverage)
endif()

target_link_libraries(test gcov)
