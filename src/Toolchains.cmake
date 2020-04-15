cmake_minimum_required(VERSION 3.0)


set(CURRENT_TOOLCHAIN "HOST")

set(HOST_C_OUTPUT_EXTENSION ${CMAKE_C_OUTPUT_EXTENSION})
set(HOST_C_COMPILE_OBJECT ${CMAKE_C_COMPILE_OBJECT})
set(HOST_STATIC_LIBRARY_PREFIX ${CMAKE_STATIC_LIBRARY_PREFIX})
set(HOST_STATIC_LIBRARY_SUFFIX ${CMAKE_STATIC_LIBRARY_SUFFIX})
set(HOST_C_CREATE_STATIC_LIBRARY ${CMAKE_C_CREATE_STATIC_LIBRARY})

set(DEVICE_C_OUTPUT_EXTENSION ".ll")
set(DEVICE_C_COMPILE_OBJECT "\
    clang \
    -S -emit-llvm --target=spir64-unknown-unknown -O3 \
    <DEFINES> <INCLUDES> <FLAGS> \
    -o <OBJECT> -c <SOURCE> \
")
set(DEVICE_STATIC_LIBRARY_PREFIX "")
set(DEVICE_STATIC_LIBRARY_SUFFIX ".ll")
set(DEVICE_C_CREATE_STATIC_LIBRARY
    "llvm-link -S <LINK_FLAGS> <OBJECTS> -o <TARGET>"
    "opt -S -O3 <TARGET> -o <TARGET>"
    "llvm-opencl <TARGET> -o <TARGET>.gen.cl"
    "xxd -i <TARGET>.gen.cl <TARGET>.gen.cl.h"
)

macro(use_host_toolchain)
  if (${CURRENT_TOOLCHAIN} STREQUAL "DEVICE")
    # Save current device flags
    set(DEVICE_C_FLAGS ${CMAKE_C_FLAGS} CACHE STRING "GCC flags for the device toolchain." FORCE)

    # Change toolchain
    set(CMAKE_C_OUTPUT_EXTENSION ${HOST_C_OUTPUT_EXTENSION})
    set(CMAKE_C_COMPILE_OBJECT ${HOST_C_COMPILE_OBJECT})
    set(CMAKE_STATIC_LIBRARY_PREFIX ${HOST_STATIC_LIBRARY_PREFIX})
    set(CMAKE_STATIC_LIBRARY_SUFFIX ${HOST_STATIC_LIBRARY_SUFFIX})
    set(CMAKE_C_CREATE_STATIC_LIBRARY ${HOST_C_CREATE_STATIC_LIBRARY})

    set(CURRENT_TOOLCHAIN "HOST" CACHE STRING "Which toolchain we are using." FORCE)
  endif()
endmacro()


macro(use_device_toolchain)
  if (${CURRENT_TOOLCHAIN} STREQUAL "HOST")
    # Save current host flags
    set(HOST_C_FLAGS ${CMAKE_C_FLAGS} CACHE STRING "GCC flags for the host toolchain." FORCE)

    # Change toolchain
    set(CMAKE_C_OUTPUT_EXTENSION ${DEVICE_C_OUTPUT_EXTENSION})
    set(CMAKE_C_COMPILE_OBJECT ${DEVICE_C_COMPILE_OBJECT})
    set(CMAKE_STATIC_LIBRARY_PREFIX ${DEVICE_STATIC_LIBRARY_PREFIX})
    set(CMAKE_STATIC_LIBRARY_SUFFIX ${DEVICE_STATIC_LIBRARY_SUFFIX})
    set(CMAKE_C_CREATE_STATIC_LIBRARY ${DEVICE_C_CREATE_STATIC_LIBRARY})

    set(CURRENT_TOOLCHAIN "DEVICE" CACHE STRING "Which toolchain we are using." FORCE)
  endif()
endmacro()


function(set_device_source_properties)
  #use_device_toolchain()
  
  set(SRC ${ARGN})

  set_source_files_properties(${SRC} PROPERTIES LANGUAGE C)

  set(CL_SRC ${SRC})
  list(FILTER CL_SRC INCLUDE REGEX ".+\\.cl$")
  set_source_files_properties(
      ${CL_SRC} PROPERTIES
      COMPILE_FLAGS "-x cl -std=cl1.2 -DOPENCL -Xclang -finclude-default-header"
  )

  set(C_SRC ${SRC})
  list(FILTER C_SRC INCLUDE REGEX ".+\\.c$")
  #set_source_files_properties(${C_SRC} PROPERTIES COMPILE_FLAGS "-std=c99")

  set(CPP_SRC ${SRC})
  list(FILTER CPP_SRC INCLUDE REGEX ".+\\.(cpp|cxx|cc)$")
  set_source_files_properties(${CPP_SRC} PROPERTIES COMPILE_FLAGS "-std=c++14")
endfunction()

function(set_device_target_properties TARGET)
  #use_device_toolchain()

  set_target_properties(${TARGET} PROPERTIES LINKER_LANGUAGE C)
endfunction()
