# Find the compiler
find_program(
    CMAKE_CL_COMPILER
        NAMES "clang"
        HINTS "${CMAKE_SOURCE_DIR}"
        DOC "CL compiler"
)
mark_as_advanced(CMAKE_CL_COMPILER)

set(CMAKE_CL_SOURCE_FILE_EXTENSIONS cl;c;cc;cpp;cxx)
set(CMAKE_CL_OUTPUT_EXTENSION .ll)
set(CMAKE_CL_COMPILER_ENV_VAR "CL")

# Configure variables set in this file for fast reload later on
configure_file(
    ${CMAKE_CURRENT_LIST_DIR}/CMakeCLCompiler.cmake.in
    ${CMAKE_PLATFORM_INFO_DIR}/CMakeCLCompiler.cmake
)
