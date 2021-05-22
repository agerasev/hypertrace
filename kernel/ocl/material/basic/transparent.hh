#pragma once

typedef void Transparent;

#define $Self Transparent
#define $self transparent
#include <material/interface.inl>
#undef $Self
#undef $self

#ifndef HOST
#include "transparent.cc"
#endif // !HOST
