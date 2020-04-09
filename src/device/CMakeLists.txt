cmake_minimum_required(VERSION 3.0)
project(device C)

set(CMAKE_LINK_LIBRARY_SUFFIX "")
set(CMAKE_STATIC_LIBRARY_PREFIX "")
set(CMAKE_STATIC_LIBRARY_SUFFIX ".ll")

set(CMAKE_C_CREATE_STATIC_LIBRARY
    "llvm-link -S <LINK_FLAGS> <OBJECTS> -o <TARGET>"
    "opt -S -O3 <TARGET> -o <TARGET>"
    "llvm-opencl <TARGET>"
)
set(CMAKE_C_CREATE_SHARED_LIBRARY "false")
set(CMAKE_C_CREATE_SHARED_MODULE "false")
set(CMAKE_C_LINK_EXECUTABLE "false")

set(CMAKE_C_COMPILER "clang")
set(CMAKE_C_FLAGS "\
-S -emit-llvm --target=spir64-unknown-unknown \
-Xclang -finclude-default-header \
-O3 -g -Wall\
")

set(DEVICE_SRC
    "math.cl"
    "builtin.cl"
    "render.cl"
)
include_directories(
    "."
    "../common"
)

add_library(${PROJECT_NAME} STATIC ${COMMON_SRC} ${DEVICE_SRC})
target_compile_definitions(${PROJECT_NAME} PUBLIC "-DDEVICE")
set_source_files_properties(${COMMON_SRC} ${DEVICE_SRC} PROPERTIES LANGUAGE C)
set_source_files_properties(${COMMON_SRC} PROPERTIES COMPILE_FLAGS "-std=c++14")
set_source_files_properties(${DEVICE_SRC} PROPERTIES COMPILE_FLAGS "-x cl -std=cl1.2")
set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE C)
