cmake_minimum_required(VERSION 3.0)
project(kernel)

set(CMAKE_C_FLAGS "-g -Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++17 -fno-exceptions")

add_definitions(
    "-DDEBUG"
    "-DHOST"
    "-DUNITTEST"
)

include_directories("src")

set(SRC
    "src/types.cc"
    "src/math.cc"
    "src/macros_test.cpp"

    "src/algebra/real.cc"
    "src/algebra/vector.cc"
    "src/algebra/complex.cc"
    "src/algebra/quaternion.cc"
    "src/algebra/matrix_real.cc"
    "src/algebra/matrix_complex.cc"
    "src/algebra/matrix_test.cpp"
    "src/algebra/moebius.cc"
    "src/algebra/linear.cc"
    "src/algebra/affine.cc"
    "src/algebra/rotation.cc"
    "src/algebra/color.cc"

    "src/random.cc"

    "src/geometry/euclidean.cc"
    "src/geometry/hyperbolic.cc"
    "src/geometry/ray_test.cpp"

    "src/render/light.cc"

    "src/shape/eu/shapes.cc"
    "src/shape/hy/plane.cc"
    "src/shape/hy/horosphere.cc"

    "src/material/material.cc"
    "src/material/modifier_test.cpp"
)

enable_testing()
add_subdirectory("/usr/src/gtest" ${CMAKE_BINARY_DIR}/gtest)
include_directories(${GTEST_INCLUDE_DIR})

macro(add_test_target TARGET)
    add_executable(${TARGET} ${SRC} "test/unittest.cpp")
    target_link_libraries(${TARGET} pthread gtest gtest_main)
    add_test(${TARGET} ${TARGET})
    install(TARGETS ${TARGET} RUNTIME DESTINATION "bin")
endmacro()

set(PROJECT_FLOAT "${PROJECT_NAME}_float")
add_test_target(${PROJECT_FLOAT})

set(PROJECT_DOUBLE "${PROJECT_NAME}_double")
add_test_target(${PROJECT_DOUBLE})
target_compile_definitions(${PROJECT_DOUBLE} PUBLIC "DOUBLE_SUPPORT")
