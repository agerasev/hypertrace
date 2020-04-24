cmake_minimum_required(VERSION 3.0)
project(device)

use_device_toolchain()

set(DEVICE "${BASE}_device")
set(DEVICE ${DEVICE} PARENT_SCOPE)

set(SRC
    "kernel.cl"
    "main.cpp"
)

add_library(${DEVICE} STATIC ${SRC})
target_include_directories(${DEVICE} PUBLIC
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_SOURCE_DIR}/device"
    "${CMAKE_CURRENT_SOURCE_DIR}/../common"
)
set_device_source_properties(${SRC})
set_device_target_properties(${DEVICE})
target_link_libraries(${DEVICE} device)
