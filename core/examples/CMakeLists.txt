cmake_minimum_required(VERSION 3.0)

#add_subdirectory(empty)
#add_subdirectory(hyperbolic)
#add_subdirectory(raymarching)

#add_dependencies(main examples)

file(GLOB EXAMPLES "${CMAKE_CURRENT_SOURCE_DIR}/*.cpp")
foreach(EXAMPLE_PATH ${EXAMPLES})
    get_filename_component(EXAMPLE_NAME ${EXAMPLE_PATH} NAME_WE)
    add_executable(${EXAMPLE_NAME} ${EXAMPLE_PATH})
    target_link_libraries(${EXAMPLE_NAME}
        ${PROJECT_NAME}
        ${RSTD_LIBS}
        ${OPENCL_LIBS}
        ${SDL_LIBS}
    )
endforeach()
