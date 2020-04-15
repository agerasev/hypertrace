cmake_minimum_required(VERSION 3.0)
project(gen)

use_host_toolchain()

set(GENERATE "${BASE}_gen")
set(GENERATE ${GENERATE} PARENT_SCOPE)

add_executable(${GENERATE} "main.cpp")
target_include_directories(${GENERATE} PUBLIC
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_SOURCE_DIR}/host"
    "${CMAKE_CURRENT_SOURCE_DIR}"
)
target_link_libraries(${GENERATE} host)
add_custom_command(TARGET ${GENERATE} POST_BUILD
    COMMAND "${CMAKE_CURRENT_BINARY_DIR}/${GENERATE}" ARGS
        "${CMAKE_CURRENT_BINARY_DIR}/config.h"
)
