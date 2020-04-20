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
    "${CMAKE_SOURCE_DIR}/host/builtins/math.cpp"
    ${COMMON_SRC}
)
set(COMMON_SRC ${COMMON_SRC} PARENT_SCOPE)

set(HOST_SRC
    "opencl/search.cpp"
    "opencl/opencl.cpp"
    "sdl/base.cpp"
    "sdl/controller.cpp"
    "sdl/viewer.cpp"
    #"scenario.cpp"
)

include_directories(
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_CURRENT_SOURCE_DIR}"
)

add_library(${PROJECT_NAME} OBJECT ${COMMON_SRC} ${HOST_SRC})
target_compile_definitions(${PROJECT_NAME} PUBLIC
    "-DHOST"
    "-DCL_TARGET_OPENCL_VERSION=120"
)

target_link_libraries(${PROJECT_NAME} OpenCL SDL2 SDL2_image)
