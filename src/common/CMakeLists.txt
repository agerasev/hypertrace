cmake_minimum_required(VERSION 3.0)

set(COMMON_SRC
    "${CMAKE_SOURCE_DIR}/common/container/union.cpp"
    "${CMAKE_SOURCE_DIR}/common/container/tuple.cpp"
    "${CMAKE_SOURCE_DIR}/common/container/variant.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/vector.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/complex.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/matrix.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/linear.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/affine.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/rotation.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/moebius.cpp"
    #"${CMAKE_SOURCE_DIR}/common/geometry/euclidean.cpp"
    "${CMAKE_SOURCE_DIR}/common/geometry/hyperbolic.cpp"
    "${CMAKE_SOURCE_DIR}/common/object/hyperbolic/horosphere.cpp"
    "${CMAKE_SOURCE_DIR}/common/object/hyperbolic/plane.cpp"
    "${CMAKE_SOURCE_DIR}/common/object/material.cpp"
    "${CMAKE_SOURCE_DIR}/common/random.cpp"
PARENT_SCOPE)
