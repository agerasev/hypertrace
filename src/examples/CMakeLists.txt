cmake_minimum_required(VERSION 3.0)
project(examples)

add_custom_target(${PROJECT_NAME})

include_directories(
    "${CMAKE_SOURCE_DIR}/common"
    "${CMAKE_SOURCE_DIR}/host"
    "${CMAKE_CURRENT_SOURCE_DIR}"
    "${CMAKE_BINARY_DIR}/device"
)

# Examples

add_executable(main "main.cpp")
target_link_libraries(main host)

#file(GLOB EXAMPLES "examples/*.cpp")
#foreach(EXAMPLE_PATH ${EXAMPLES})
#    get_filename_component(EXAMPLE_NAME ${EXAMPLE_PATH} NAME_WE)
#    add_executable(${EXAMPLE_NAME} ${EXAMPLE_PATH})
#    target_link_libraries(${EXAMPLE_NAME} ${PROJECT_NAME})
#endforeach()

add_dependencies(${PROJECT_NAME} main)
