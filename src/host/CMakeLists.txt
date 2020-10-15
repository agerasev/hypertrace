set(HOST_OPENCL_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/opencl.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/search.cpp"
)
set(HOST_OPENCL_SRC ${HOST_OPENCL_SRC} PARENT_SCOPE)

set(HOST_SRC
    ${HOST_OPENCL_SRC}

    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/base.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/controller.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/image.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/viewer.cpp"

    "${CMAKE_CURRENT_SOURCE_DIR}/renderer.cpp"
)
set(HOST_SRC ${HOST_SRC} PARENT_SCOPE)
