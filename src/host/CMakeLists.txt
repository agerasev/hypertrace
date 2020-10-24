set(HOST_OPENCL_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/opencl.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/opencl/search.cpp"
    
    "${CMAKE_CURRENT_SOURCE_DIR}/aligned.cpp"
)
set(HOST_OPENCL_SRC ${HOST_OPENCL_SRC} PARENT_SCOPE)

set(HOST_DYNTYPE_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/source.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/primitive.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/array.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/vector.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/tuple.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/struct.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/variant.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/static_tuple.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/dyntype/static_struct.cpp"
)
set(HOST_DYNTYPE_SRC ${HOST_DYNTYPE_SRC} PARENT_SCOPE)

set(HOST_SRC
    ${HOST_OPENCL_SRC}
    ${HOST_DYNTYPE_SRC}

    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/base.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/controller.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/image.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/sdl/viewer.cpp"

    "${CMAKE_CURRENT_SOURCE_DIR}/renderer.cpp"
)
set(HOST_SRC ${HOST_SRC} PARENT_SCOPE)
