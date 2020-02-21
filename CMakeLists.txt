cmake_minimum_required(VERSION 2.6)
project(hypertrace)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -std=c++14 -Wall -Wextra")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -Wall -Wextra")

set(SRC
    "src/common/vectype.hpp"
    "src/common/vectype.cpp"
    "src/common/types.hh"
    "src/common/types.cc"
    "src/common/algebra/real.hh"
#    "src/common/algebra/real.cc"
    "src/common/algebra/complex.hh"
    "src/common/algebra/complex.cc"
    "src/common/algebra/quaternion.hh"
    "src/common/algebra/quaternion.cc"
    "src/common/algebra/moebius.hh"
    "src/common/algebra/moebius.cc"
    "src/common/geometry/hyperbolic.hh"
    "src/common/geometry/hyperbolic.cc"
)
include_directories(
    "src/host"
    "src/common"
)


# Main target

add_executable(${PROJECT_NAME}
    ${SRC}
    "src/host/opencl/include.hpp"
    "src/host/opencl/opencl.hpp"
    "src/host/viewer.hpp"
    "src/host/main.cpp"
)
target_compile_definitions(${PROJECT_NAME} PRIVATE
    -DCL_TARGET_OPENCL_VERSION=120
    -DOPENCL_INTEROP
)
target_link_libraries(${PROJECT_NAME} OpenCL SDL2)


# Test target

add_executable(test
    ${SRC}
    "src/host/test.cpp"
)
target_compile_definitions(test PRIVATE
    -DUNIT_TEST
)

if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    target_compile_options(test PRIVATE
        --coverage
    )
endif()

target_link_libraries(test gcov)
