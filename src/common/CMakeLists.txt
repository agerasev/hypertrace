cmake_minimum_required(VERSION 3.0)

set(COMMON_SRC
    "${CMAKE_SOURCE_DIR}/common/algebra/vector.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/complex.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/matrix.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/moebius.cpp"
    "${CMAKE_SOURCE_DIR}/common/algebra/rotation.cpp"
    "${CMAKE_SOURCE_DIR}/common/geometry/hyperbolic.cpp"
    #"${CMAKE_SOURCE_DIR}/common/geometry/hyperbolic/ray.cc"
    #"${CMAKE_SOURCE_DIR}/common/geometry/hyperbolic/plane.cc"
    #"${CMAKE_SOURCE_DIR}/common/geometry/hyperbolic/horosphere.cc"
    #"${CMAKE_SOURCE_DIR}/common/random.cc"
    #"${CMAKE_SOURCE_DIR}/common/material.cc"
    #"${CMAKE_SOURCE_DIR}/common/object.cc"
    #"${CMAKE_SOURCE_DIR}/common/view.cc"
PARENT_SCOPE)
