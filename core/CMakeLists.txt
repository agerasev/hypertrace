if(USE_PCH)
    cmake_minimum_required(VERSION 3.16)
    message(STATUS "Precompiled headers ON")
else()
    cmake_minimum_required(VERSION 3.0)
    message(STATUS "Precompiled headers OFF")
endif()

project(kernel)


set(CMAKE_C_FLAGS "-g -Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++17 -fno-exceptions")

add_subdirectory("../rstd" "__/rstd")
include_directories("../rstd/src")

add_definitions(
    "-DDEBUG"
    "-DHOST"
    "-DTEST"
    "-DTEST_UNIT"
    "-DDOUBLE_SUPPORT"
)


# Library

include_directories("src")

set(PCH
    "src/algebra/vector_base.hpp"
    "src/algebra/vector_math.hpp"
    "src/algebra/vector_struct.hpp"
)

set(SRC
    "src/types.cc"
    "src/math.cc"

    "src/algebra/real.cc"
    "src/algebra/vector.cc"
    "src/algebra/complex.cc"
    "src/algebra/quaternion.cc"
    #"src/algebra/matrix_real.cc"
    #"src/algebra/matrix_complex.cc"
    #"src/algebra/matrix_test.cpp"
    #"src/algebra/moebius.cc"
    #"src/algebra/linear.cc"
    #"src/algebra/affine.cc"
    #"src/algebra/rotation.cc"
    #"src/algebra/color.cc"

    #"src/random.cc"

    #"src/geometry/euclidean.cc"
    #"src/geometry/hyperbolic.cc"
    #"src/geometry/ray.cc"

    #"src/shape/euclidean/shapes.cc"
    #"src/shape/hyperbolic/plane.cc"
    #"src/shape/hyperbolic/horosphere.cc"

    #"src/material/material.cc"
    #"src/material/modifier.cpp"

    #"src/render/view.cc"
    #"src/render/light.cc"
)

add_library(${PROJECT_NAME} OBJECT ${SRC})
if(USE_PCH)
    target_precompile_headers(${PROJECT_NAME} PUBLIC ${PCH})
endif()
target_link_libraries(${PROJECT_NAME} pthread rstd)


# Tests

set(PROJECT_TEST "${PROJECT_NAME}_test")
add_executable(${PROJECT_TEST} "test/unittest.cpp")
if(USE_PCH)
    target_precompile_headers(${PROJECT_TEST} PRIVATE ${PCH})
endif()
target_link_libraries(${PROJECT_TEST} ${PROJECT_NAME} gtest pthread rstd)

enable_testing()
add_test(${PROJECT_NAME} ${PROJECT_TEST})
