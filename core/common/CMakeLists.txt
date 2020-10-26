set(COMMON_PCH
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector_base.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector_math.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/vector_struct.hpp"
PARENT_SCOPE)

set(COMMON_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/types.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/math.cc"

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
    "${CMAKE_CURRENT_SOURCE_DIR}/algebra/color.cc"

    "${CMAKE_CURRENT_SOURCE_DIR}/random.cc"

    "${CMAKE_CURRENT_SOURCE_DIR}/geometry/euclidean.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/geometry/hyperbolic.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/geometry/ray.cc"

    "${CMAKE_CURRENT_SOURCE_DIR}/shape/euclidean/shapes.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/shape/hyperbolic/plane.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/shape/hyperbolic/horosphere.cc"

    "${CMAKE_CURRENT_SOURCE_DIR}/material/material.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/material/modifier.cpp"

    "${CMAKE_CURRENT_SOURCE_DIR}/render/view.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/render/light.cc"
PARENT_SCOPE)
