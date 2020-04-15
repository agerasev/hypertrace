cmake_minimum_required(VERSION 3.0)
project(device)

use_device_toolchain()

set(DEVICE "${BASE}_device")
set(DEVICE ${DEVICE} PARENT_SCOPE)

add_library(${DEVICE} STATIC "main.cpp")
target_include_directories(${DEVICE} PUBLIC
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_SOURCE_DIR}/device"
)
target_compile_definitions(${DEVICE} PUBLIC "-DDEVICE")
set_device_source_properties("main.cpp")
set_device_target_properties(${DEVICE})
target_link_libraries(${DEVICE} device)
add_dependencies(${DEVICE} ${GENERATE})
