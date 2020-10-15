if(USE_PCH)
    cmake_minimum_required(VERSION 3.16)
    message(STATUS "Precompiled headers ON")
else()
    cmake_minimum_required(VERSION 3.0)
    message(STATUS "Precompiled headers OFF")
endif()

project(hypertrace)

set(CMAKE_C_FLAGS "-DHOST -Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++17 -fno-exceptions")

add_subdirectory("submodules/rstd")
include_directories("submodules/rstd/src")

add_subdirectory("submodules/includer")
include_directories("submodules/includer/src")

add_subdirectory("src/common")
add_subdirectory("src/host")

include_directories(
    "${CMAKE_SOURCE_DIR}/src"
)
add_definitions(
    "-DDEBUG"
    "-DHOST"
)
set(RSTD_LIBS
    pthread
    rstd
)
set(OPENCL_LIBS
    includer
    OpenCL
)
set(SDL_LIBS
    SDL2
    SDL2_image
)

add_library(${PROJECT_NAME} OBJECT
    ${COMMON_SRC}
    ${HOST_SRC}
)
target_compile_definitions(${PROJECT_NAME} PUBLIC
    "-DINTEROP"
    "-DCL_TARGET_OPENCL_VERSION=120"
    "-DCL_USE_DEPRECATED_OPENCL_1_2_APIS"
)
if(USE_PCH)
    target_precompile_headers(${PROJECT_NAME} PUBLIC
        ${COMMON_PCH}
    )
endif()
target_link_libraries(${PROJECT_NAME}
    ${RSTD_LIBS}
    ${OPENCL_LIBS}
    ${SDL_LIBS}
)

add_subdirectory("src/examples")
add_subdirectory("src/test")
