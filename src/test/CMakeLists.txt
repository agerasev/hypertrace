cmake_minimum_required(VERSION 3.0)
project(test)


include_directories(
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_SOURCE_DIR}/host"
    "${CMAKE_CURRENT_SOURCE_DIR}"
)

# Unit tests

add_executable(test ${COMMON_SRC} "unit_test.cpp")
target_compile_definitions(test PRIVATE
    "-DHOST"
    "-DUNIT_TEST"
)
