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
-O3 -Wall\
")

set(DEVICE_SRC
    "builtins/math.cl"
    "builtins/vector.cl"
    "render.cl"
    "trace.cpp"
)
include_directories(
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_CURRENT_SOURCE_DIR}"
)

set(SRC ${COMMON_SRC} ${DEVICE_SRC})

add_library(${PROJECT_NAME} STATIC ${SRC})
target_compile_definitions(${PROJECT_NAME} PUBLIC "-DDEVICE")
set_source_files_properties(${SRC} PROPERTIES LANGUAGE C)

set(CL_SRC ${SRC})
LIST(FILTER CL_SRC INCLUDE REGEX ".+\\.cl$")
set_source_files_properties(${CL_SRC} PROPERTIES COMPILE_FLAGS "-x cl -std=cl1.2 -DOPENCL")

set(C_SRC ${SRC})
LIST(FILTER C_SRC INCLUDE REGEX ".+\\.c$")
#set_source_files_properties(${C_SRC} PROPERTIES COMPILE_FLAGS "-std=c99")

set(CPP_SRC ${SRC})
LIST(FILTER CPP_SRC INCLUDE REGEX ".+\\.(cpp|cxx|cc)$")
set_source_files_properties(${CPP_SRC} PROPERTIES COMPILE_FLAGS "-std=c++14")

set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE C)
