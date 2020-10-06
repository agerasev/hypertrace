cmake_minimum_required(VERSION 3.0)
project(test)

add_executable(test_unit
    ${COMMON_SRC}
    "test_unit.cpp"
)
target_compile_definitions(test_unit PRIVATE
    "-DTEST_UNIT"
)
target_link_libraries(test_unit pthread rstd)

add_executable(test_catch
    ${COMMON_SRC}
    "test_catch.cpp"
)
target_compile_definitions(test_catch PRIVATE
    "-DTEST_CATCH"
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
target_link_libraries(test_dev pthread rstd includer OpenCL)
