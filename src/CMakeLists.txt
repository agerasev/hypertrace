cmake_minimum_required(VERSION 3.0)
project(hypertrace)

include("Toolchains.cmake")

add_subdirectory(common)
add_subdirectory(device)
add_subdirectory(host)
add_subdirectory(examples)
add_subdirectory(test)

add_dependencies(host device)
add_dependencies(main host)
