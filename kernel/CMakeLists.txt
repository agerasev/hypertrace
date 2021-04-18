cmake_minimum_required(VERSION 3.0)
project(kernel)

set(CMAKE_C_FLAGS "-g -Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++17 -fno-exceptions")

add_definitions(
    "-DDEBUG"
    "-DHOST"
    "-DUNITTEST"
    "-DVARIADIC_MACROS"
)

include_directories("ocl")

set(SRC
    "ocl/types.cc"
    "ocl/math.cc"
    "ocl/macros_test.cpp"

    "ocl/algebra/real.cc"
    "ocl/algebra/vector.cc"
    "ocl/algebra/complex.cc"
    "ocl/algebra/quaternion.cc"
    "ocl/algebra/matrix_real.cc"
    "ocl/algebra/matrix_complex.cc"
    "ocl/algebra/matrix_test.cpp"
    "ocl/algebra/moebius.cc"
    "ocl/algebra/linear.cc"
    "ocl/algebra/affine.cc"
    "ocl/algebra/rotation.cc"
    "ocl/algebra/color.cc"

    "ocl/random.cc"

    "ocl/geometry/euclidean.cc"
    "ocl/geometry/hyperbolic.cc"
    "ocl/geometry/ray_test.cpp"

    "ocl/render/light.cc"

    "ocl/shape/eu/shapes.cc"
    "ocl/shape/hy/plane.cc"
    "ocl/shape/hy/horosphere.cc"

    "ocl/material/material.cc"
    "ocl/material/modifier_test.cpp"
)

enable_testing()
add_subdirectory("/usr/src/gtest" ${CMAKE_BINARY_DIR}/gtest)
include_directories(${GTEST_INCLUDE_DIR})

macro(add_test_target TARGET)
    add_executable(${TARGET} ${SRC} "ocl/test.cpp")
    target_link_libraries(${TARGET} pthread gtest gtest_main)
    add_test(${TARGET} ${TARGET})
    install(TARGETS ${TARGET} RUNTIME DESTINATION "bin")
endmacro()

set(PROJECT_FLOAT "${PROJECT_NAME}_f32")
add_test_target(${PROJECT_FLOAT})

set(PROJECT_DOUBLE "${PROJECT_NAME}_f64")
add_test_target(${PROJECT_DOUBLE})
target_compile_definitions(${PROJECT_DOUBLE} PUBLIC "DOUBLE_PRECISION")
