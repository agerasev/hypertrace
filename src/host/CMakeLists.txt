cmake_minimum_required(VERSION 3.0)
project(host)

set(COMMON_FLAGS "\
    -g \
    -Wall \
    -Wextra \
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
    "math.cpp"
    ${COMMON_SRC}
)

set(HOST_SRC
    "opencl/search.hpp"
    "opencl/search.cpp"
    "opencl/opencl.hpp"
    "opencl/opencl.cpp"
    "sdl/base.hpp"
    "sdl/base.cpp"
    "sdl/controller.hpp"
    "sdl/controller.cpp"
    "sdl/viewer.hpp"
    "sdl/viewer.cpp"
    "renderer.hpp"
    "renderer.cpp"
    #"scenario.hpp"
    #"scenario.cpp"
)

include_directories(
    "."
    "../common"
)

add_library(${PROJECT_NAME} OBJECT ${COMMON_SRC} ${HOST_SRC})
target_compile_definitions(${PROJECT_NAME} PUBLIC
    "-DHOST"
    "-DCL_TARGET_OPENCL_VERSION=120"
)

target_link_libraries(${PROJECT_NAME} OpenCL SDL2 SDL2_image)

# Examples

add_executable(main "examples/main.cpp")
target_link_libraries(main ${PROJECT_NAME})

#file(GLOB EXAMPLES "examples/*.cpp")
#foreach(EXAMPLE_PATH ${EXAMPLES})
#    get_filename_component(EXAMPLE_NAME ${EXAMPLE_PATH} NAME_WE)
#    add_executable(${EXAMPLE_NAME} ${EXAMPLE_PATH})
#    target_link_libraries(${EXAMPLE_NAME} ${PROJECT_NAME})
#endforeach()


# Tests

add_executable(test ${COMMON_SRC} "tests/unit_test.cpp")
target_compile_definitions(test PRIVATE
    "-DHOST"
    "-DUNIT_TEST"
)
