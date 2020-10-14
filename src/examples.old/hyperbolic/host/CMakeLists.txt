cmake_minimum_required(VERSION 3.0)
project(host)

use_host_toolchain()

set(MAIN_SRC
    "main.cpp"
    "replay.cpp"
    "horosphere.cpp"
)

include_directories(
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_SOURCE_DIR}/host"
    "${CMAKE_CURRENT_SOURCE_DIR}/../common"
    "${CMAKE_CURRENT_BINARY_DIR}/../"
)

foreach(SRC ${MAIN_SRC})
    get_filename_component(NAME ${SRC} NAME_WE)
    set(TARGET "${BASE}_${NAME}")
    add_executable(${TARGET} ${SRC})
    target_link_libraries(${TARGET} host)
    add_dependencies(${TARGET} ${DEVICE})
endforeach()
