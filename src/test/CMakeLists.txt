project(test)

add_executable(test_unit
    ${COMMON_SRC}
    "test_unit.cpp"
)
if(USE_PCH)
    target_precompile_headers(test_unit PRIVATE
        ${COMMON_PCH}
    )
endif()
target_compile_definitions(test_unit PRIVATE
    "-DTEST_UNIT"
)
target_link_libraries(test_unit pthread rstd)

add_executable(test_dev
    ${COMMON_SRC}
    ${HOST_OPENCL_SRC}
    "devtest.cpp"
    "test_dev.cpp"
)
target_compile_definitions(test_dev PRIVATE
    "-DTEST_DEV"
    "-DINTEROP"
    "-DCL_TARGET_OPENCL_VERSION=120"
    "-DCL_USE_DEPRECATED_OPENCL_1_2_APIS"
)
if(USE_PCH)
    target_precompile_headers(test_unit PRIVATE
        ${COMMON_PCH}
    )
endif()
target_link_libraries(test_dev pthread rstd includer OpenCL)
