cmake_minimum_required(VERSION 3.0)

project("common")

set(SRC
    "src/types.hpp"
    "src/math.hpp"
)
add_library(${PROJECT_NAME} OBJECT ${SRC})
target_include_directories(${PROJECT_NAME} PUBLIC
    "src"
    "../thirdparty/clcxx/include"
)
