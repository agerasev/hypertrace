set(HOST_OPENCL_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/opencl.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/search.cpp"
PARENT_SCOPE)

set(HOST_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/base.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/controller.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/image.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/viewer.cpp"

    #"${CMAKE_CURRENT_SOURCE_DIR}/renderer.cpp"
PARENT_SCOPE)
