cmake_minimum_required(VERSION 3.12)

set(COMMON_SRC
    #"${CMAKE_SOURCE_DIR}/host/builtins/math.cpp"
    ${COMMON_SRC}
PARENT_SCOPE)

set(HOST_SRC
    "${CMAKE_SOURCE_DIR}/host/empty.cpp"
PARENT_SCOPE)
