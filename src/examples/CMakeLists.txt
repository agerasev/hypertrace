cmake_minimum_required(VERSION 3.0)
project(examples)

add_custom_target(${PROJECT_NAME})

add_subdirectory(hyperbolic)
add_subdirectory(raymarching)

#add_dependencies(main examples)

#file(GLOB EXAMPLES "examples/*.cpp")
#foreach(EXAMPLE_PATH ${EXAMPLES})
#    get_filename_component(EXAMPLE_NAME ${EXAMPLE_PATH} NAME_WE)
#    add_executable(${EXAMPLE_NAME} ${EXAMPLE_PATH})
#    target_link_libraries(${EXAMPLE_NAME} ${PROJECT_NAME})
#endforeach()
