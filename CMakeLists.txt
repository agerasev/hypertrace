cmake_minimum_required(VERSION 3.0)
project(hypertrace)

set(COMMON_FLAGS "\
    -g \
    -Wall \
    -Wextra \
    -Wno-unused-parameter \
")
set(CMAKE_C_FLAGS "\
    ${CMAKE_C_FLAGS} \
    ${COMMON_FLAGS} \
")
set(CMAKE_CXX_FLAGS "\
    ${CMAKE_CXX_FLAGS} \
    ${COMMON_FLAGS} \
    -std=c++14 \
")

set(COMMON_SRC
    "src/host/vectype.hpp"
    "src/host/vectype.cpp"
    "src/common/types.hh"
    "src/common/types.cc"
    "src/common/algebra/real.hh"
    "src/common/algebra/complex.hh"
    "src/common/algebra/complex.cc"
    "src/common/algebra/quaternion.hh"
    "src/common/algebra/quaternion.cc"
    "src/common/algebra/moebius.hh"
    "src/common/algebra/moebius.cc"
    "src/common/algebra/rotation.hh"
    "src/common/algebra/rotation.cc"
    "src/common/geometry/hyperbolic.hh"
    "src/common/geometry/hyperbolic.cc"
    "src/common/geometry/hyperbolic/ray.hh"
    "src/common/geometry/hyperbolic/ray.cc"
    "src/common/geometry/hyperbolic/plane.hh"
    "src/common/geometry/hyperbolic/plane.cc"
    "src/common/geometry/hyperbolic/horosphere.hh"
    "src/common/geometry/hyperbolic/horosphere.cc"
    "src/common/random.hh"
    "src/common/random.cc"
    "src/common/material.hh"
    "src/common/material.cc"
    "src/common/object.hh"
    "src/common/object.cc"
    "src/common/view.hh"
    "src/common/view.cc"
)
set(HOST_SRC
    ${COMMON_SRC}
    "src/host/vectype.hpp"
    "src/host/vectype.cpp"
    "src/host/opencl/crawler.hpp"
    "src/host/opencl/crawler.cpp"
    "src/host/opencl/include.hpp"
    "src/host/opencl/include.cpp"
    "src/host/opencl/opencl.hpp"
    "src/host/opencl/opencl.cpp"
    "src/host/sdl/base.hpp"
    "src/host/sdl/base.cpp"
    "src/host/sdl/controller.hpp"
    "src/host/sdl/controller.cpp"
    "src/host/sdl/viewer.hpp"
    "src/host/sdl/viewer.cpp"
    "src/host/renderer.hpp"
    "src/host/renderer.cpp"
    "src/host/scenario.hpp"
    "src/host/scenario.cpp"
)
include_directories(
    "src/host"
    "src/common"
)

# Main target

add_executable(${PROJECT_NAME}
    ${HOST_SRC}
    "src/host/main.cpp"
)
target_compile_definitions(${PROJECT_NAME} PRIVATE
    "-DCL_TARGET_OPENCL_VERSION=120"
    "-DOPENCL_INTEROP"
)
target_link_libraries(${PROJECT_NAME} OpenCL SDL2)


# Flight target

add_executable(replay
    ${HOST_SRC}
    "src/host/replay.cpp"
)
target_compile_definitions(replay PRIVATE
    "-DCL_TARGET_OPENCL_VERSION=120"
    "-DOPENCL_INTEROP"
)
target_link_libraries(replay OpenCL SDL2)



# Test target

add_executable(test
    ${COMMON_SRC}
    "src/host/test.cpp"
)
target_compile_definitions(test PRIVATE
    "-DUNIT_TEST"
)

if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    target_compile_options(test PRIVATE
        "--coverage"
    )
endif()

target_link_libraries(test gcov)
