cmake_minimum_required(VERSION 3.0)

set(COMMON_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/types.hh"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/real.hh"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/real.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector_base.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector_struct.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector_math.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector.hh"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/complex.hh"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/complex.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/quaternion.hh"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/quaternion.cc"
PARENT_SCOPE)
