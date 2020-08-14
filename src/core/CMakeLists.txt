cmake_minimum_required(VERSION 3.0)

set(CORE_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/container/tuple.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/container/union.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/container/variant.cpp"

    "${CMAKE_CURRENT_SOURCE_DIR}/prelude.cpp"
PARENT_SCOPE)
