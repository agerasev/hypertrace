cmake_minimum_required(VERSION 3.0)
project(device C)

add_subdirectory("common")

set(CMAKE_C_COMPILER clang)
set(CMAKE_C_FLAGS "\
    -S -emit-llvm \
    --target=spir-unknown-unknown \
    -x cl -std=cl1.2 \
    -Xclang -finclude-default-header \
    -O3 \
")
set(CMAKE_LINKER llvm-link)
set(CMAKE_OPT opt)
set(CMAKE_AR llvm-opencl)
SET(CMAKE_C_ARCHIVE_CREATE "<CMAKE_LINKER> -S <LINK_FLAGS> <OBJECTS> -o <TARGET>")
SET(CMAKE_C_ARCHIVE_FINISH "<CMAKE_AR> <TARGET>")

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
