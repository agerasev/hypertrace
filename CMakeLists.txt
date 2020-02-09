cmake_minimum_required(VERSION 2.6)
project(hypertrace)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -std=c++14 -Wall -Wextra")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -Wall -Wextra")

set(SRC
	src/host/opencl/opencl.hpp
	src/host/opencl/include.hpp
	src/host/viewer.hpp
	src/host/main.cpp
)

include_directories(src)

add_definitions(-DDEBUG)

add_executable(${PROJECT_NAME} ${SRC})

target_link_libraries(${PROJECT_NAME} OpenCL SDL2)
