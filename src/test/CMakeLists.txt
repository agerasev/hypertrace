cmake_minimum_required(VERSION 3.0)
project(test)

add_executable(${PROJECT_NAME}
    ${COMMON_SRC}
    "unit_test.cpp"
)
target_compile_definitions(${PROJECT_NAME} PRIVATE
    "-DUNIT_TEST"
)
