cmake_minimum_required(VERSION 3.0)
project(device)

use_device_toolchain()

set(CMAKE_C_FLAGS "\
    -Wall \
")

set(DEVICE_SRC
    "builtins/math.cl"
    "builtins/vector.cl"
    "builtins/memory.cl"
    "builtins/work.cl"
    #"render.cl"
    #"trace.cpp"
)
include_directories(
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_CURRENT_SOURCE_DIR}"
)

set(SRC ${COMMON_SRC} ${DEVICE_SRC})

add_library(${PROJECT_NAME} OBJECT ${SRC})
target_compile_definitions(${PROJECT_NAME} PUBLIC "-DDEVICE")
set_device_source_properties(${SRC})
set_device_target_properties(${PROJECT_NAME})
