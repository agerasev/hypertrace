cmake_minimum_required(VERSION 3.12)
project(empty)

set(SRC
    "main.cpp"
)

include_directories(
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_SOURCE_DIR}/host"
)

add_executable(${PROJECT_NAME} ${SRC})
target_link_libraries(${PROJECT_NAME} hypertrace)
