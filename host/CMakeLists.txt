cmake_minimum_required(VERSION 3.16)

project(hypertrace)

set(CMAKE_C_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++20 -fno-exceptions")

add_subdirectory("../common" "common")

include_directories(
    "src"
)
add_definitions(
    "-DDEBUG"
    "-DCL_TARGET_OPENCL_VERSION=300"
)
set(SRC
    "src/opencl/opencl.cpp"
    "src/opencl/search.cpp"
    "src/sdl/base.cpp"
    #"src/sdl/controller.cpp"
    "src/sdl/image.cpp"
    "src/sdl/viewer.cpp"
    "src/main.cpp"
)

add_executable(${PROJECT_NAME}
    ${SRC}
)
target_link_libraries(${PROJECT_NAME}
    common
    OpenCL
    SDL2
    SDL2_image
)

add_custom_target("kernel.h"
    xxd -n KERNEL_SPV -i ${CMAKE_BINARY_DIR}/../kernel/kernel.spv >kernel.h
)
target_include_directories(${PROJECT_NAME} PRIVATE
    "${CMAKE_BINARY_DIR}"
)
add_dependencies(${PROJECT_NAME} "kernel.h")
