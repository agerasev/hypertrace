cmake_minimum_required(VERSION 3.16)

project(hypertrace)

set(CMAKE_C_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++20 -fno-exceptions")

include_directories(
    "${CMAKE_SOURCE_DIR}"
)
add_definitions(
    "-DDEBUG"
)
set(SRC
    "src/main.cpp"
)

add_executable(${PROJECT_NAME}
    ${SRC}
)
target_link_libraries(${PROJECT_NAME}
    OpenCL
    SDL2
    SDL2_image
)
