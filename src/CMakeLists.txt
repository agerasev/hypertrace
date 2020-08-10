cmake_minimum_required(VERSION 3.0)
project(hypertrace)

set(CMAKE_C_FLAGS "-DHOST -Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++17")


add_subdirectory(common)
add_subdirectory(host)

include_directories(
    "${CMAKE_SOURCE_DIR}"
)

add_library(${PROJECT_NAME} OBJECT ${HOST_SRC})
target_compile_definitions(${PROJECT_NAME} PUBLIC
    "-DCL_TARGET_OPENCL_VERSION=120"
    "-DHOST"
    "-DINTEROP"
)

target_link_libraries(${PROJECT_NAME} OpenCL SDL2 SDL2_image)


add_subdirectory(examples)
add_subdirectory(test)
