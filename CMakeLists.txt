cmake_minimum_required(VERSION 3.0)
project(hypertrace)

set(CMAKE_C_FLAGS "-DHOST -Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++17") #-fno-exceptions

add_subdirectory("submodules/rstd")
include_directories("submodules/rstd/src")

add_subdirectory("submodules/includer")
include_directories("submodules/includer/src")

add_subdirectory("src/common")
add_subdirectory("src/host")

include_directories(
    "${CMAKE_SOURCE_DIR}/src"
)
add_library(${PROJECT_NAME} OBJECT ${HOST_SRC})
add_definitions(
    "-DDEBUG"
    "-DHOST"
)
target_compile_definitions(${PROJECT_NAME} PUBLIC
    "-DCL_TARGET_OPENCL_VERSION=120"
    "-DINTEROP"
)
target_link_libraries(${PROJECT_NAME} OpenCL SDL2 SDL2_image)

add_subdirectory("src/examples")
add_subdirectory("src/test")
