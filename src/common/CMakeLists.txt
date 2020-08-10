cmake_minimum_required(VERSION 3.0)

set(COMMON_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/real.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/complex.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/quaternion.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/matrix_real.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/matrix_complex.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/matrix_test.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/moebius.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/linear.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/affine.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/rotation.cc"

    "${CMAKE_CURRENT_SOURCE_DIR}/geometry/euclidean.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/geometry/hyperbolic.cc"

    "${CMAKE_CURRENT_SOURCE_DIR}/render/view.cc"
PARENT_SCOPE)
