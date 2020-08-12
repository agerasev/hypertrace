cmake_minimum_required(VERSION 3.0)
project(test)

add_executable(test_unit
    ${COMMON_SRC}
    "${CMAKE_SOURCE_DIR}/host/opencl/include.cpp"
    "test_unit.cpp"
)
target_compile_definitions(test_unit PRIVATE
    "-DTEST_UNIT"
)

add_executable(test_dev
    ${COMMON_SRC}
    ${HOST_OPENCL_SRC}
    "test_dev.cpp"
)
target_compile_definitions(test_dev PRIVATE
    "-DCL_TARGET_OPENCL_VERSION=120"
    "-DINTEROP"
    "-DTEST_DEV"
)
target_link_libraries(test_dev OpenCL)
