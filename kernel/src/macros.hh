#pragma once

// Concatenate `a` and `b`
#define CONCAT(a, b) a##b

// Expand `a` and `b` and concatenate the results
#define CONCAT_EX(a, b) CONCAT(a, b)

#ifndef HOST
#include "macros.cc"
#endif // HOST
