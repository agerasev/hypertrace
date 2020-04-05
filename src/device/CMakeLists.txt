cmake_minimum_required(VERSION 3.0)
project(device C)

set(CMAKE_LINK_LIBRARY_SUFFIX "")
set(CMAKE_STATIC_LIBRARY_PREFIX "")
set(CMAKE_STATIC_LIBRARY_SUFFIX ".ll")

set(CMAKE_C_COMPILER "clang")
set(CMAKE_C_COMPILER_TARGET "spir-unknown-unknown")
#set(CMAKE_INCLUDE_FLAG_C "-I")
set(CMAKE_C_COMPILE_OBJECT "\
<CMAKE_C_COMPILER> \
-x cl --std=cl1.2 \
-S -emit-llvm --target=<CMAKE_C_COMPILER_TARGET> \
-Xclang -finclude-default-header \
<DEFINES> <INCLUDES> <FLAGS> \
<SOURCE> -o <OBJECT>\
")
#set(CMAKE_C_SOURCE_FILE_EXTENSIONS cl;c;cc;cpp;cxx)
#set(CMAKE_C_OUTPUT_EXTENSION .ll)

set(CMAKE_C_CREATE_STATIC_LIBRARY
    "llvm-link -S <LINK_FLAGS> <OBJECTS> -o <TARGET>"
    "llvm-opencl <TARGET>"
)
set(CMAKE_C_CREATE_SHARED_LIBRARY "")
set(CMAKE_C_CREATE_SHARED_MODULE "")
set(CMAKE_C_LINK_EXECUTABLE "")


set(CMAKE_C_FLAGS "-O3")

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
set_source_files_properties(${COMMON_SRC} ${DEVICE_SRC} PROPERTIES LANGUAGE C)
set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE C)
