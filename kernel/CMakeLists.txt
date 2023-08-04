cmake_minimum_required(VERSION 3.16)

project(kernel)

set(COMPILE_FLAGS "-Wall -Wextra")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${COMPILE_FLAGS}")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${COMPILE_FLAGS}")

include_directories(
    "${CMAKE_SOURCE_DIR}"
)
add_definitions(
    "-DKERNEL"
)
set(SRC
    "src/main.cpp"
)

add_executable(${PROJECT_NAME}
    ${SRC}
)
