cmake_minimum_required(VERSION 3.0)
project(device NONE)

set(CMAKE_LINK_LIBRARY_SUFFIX "")
set(CMAKE_STATIC_LIBRARY_PREFIX "")
set(CMAKE_STATIC_LIBRARY_SUFFIX ".ll")

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/CMake")
enable_language(CL)

set(CMAKE_CL_FLAGS "-O3")

add_subdirectory("common")

set(DEVICE_SRC
    "render.cl"
)
include_directories(
    "."
    "../common"
)
    
add_library(${PROJECT_NAME} STATIC ${COMMON_SRC} ${DEVICE_SRC})
target_compile_definitions(${PROJECT_NAME} PUBLIC "-DOPENCL" "-DOPENCL_INTEROP")
set_source_files_properties(${COMMON_SRC} ${DEVICE_SRC} PROPERTIES LANGUAGE CL)
set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE CL)
