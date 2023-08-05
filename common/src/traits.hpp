#pragma once

#ifndef KERNEL

#include <type_traits>

#else // KERNEL

#define _LIBCPP_HAS_NO_THREADS

#pragma OPENCL EXTENSION __cl_clang_function_pointers : enable
#pragma OPENCL EXTENSION __cl_clang_variadic_functions : enable
#include <type_traits>
#pragma OPENCL EXTENSION __cl_clang_function_pointers : disable
#pragma OPENCL EXTENSION __cl_clang_variadic_functions : disable

#endif // KERNEL
