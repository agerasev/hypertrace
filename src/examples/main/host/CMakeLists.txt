cmake_minimum_required(VERSION 3.0)
project(host)

use_host_toolchain()

add_executable(${BASE} "main.cpp")
target_include_directories(${BASE} PUBLIC
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_SOURCE_DIR}/host"
    "${CMAKE_CURRENT_SOURCE_DIR}/../common"
    "${CMAKE_CURRENT_BINARY_DIR}/../"
)
target_link_libraries(${BASE} host)
add_dependencies(${BASE} ${GENERATE} ${DEVICE})
