cmake_minimum_required(VERSION 3.12)

set(HOST_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/include.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/opencl.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/search.cpp"

    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/base.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/controller.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/image.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/viewer.cpp"

    "${CMAKE_CURRENT_SOURCE_DIR}/renderer.cpp"
PARENT_SCOPE)
