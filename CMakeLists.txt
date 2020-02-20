cmake_minimum_required(VERSION 2.6)
project(hypertrace)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -std=c++14 -Wall -Wextra")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -Wall -Wextra")

set(SRC
    "src/common/algebra/real.h"
    "src/common/algebra/complex.h"
    "src/common/algebra/complex.c"
    "src/common/algebra/complex.cpp"
    "src/common/algebra/quaternion.h"
    "src/common/algebra/quaternion.c"
    "src/common/algebra/quaternion.cpp"
    "src/common/algebra/moebius.h"
    "src/common/algebra/moebius.c"
    "src/common/algebra/moebius.cpp"
    "src/common/geometry/surface.h"
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
    #${SRC}
    "src/common/algebra/vector.hh"
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
