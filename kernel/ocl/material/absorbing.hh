#pragma once

typedef void Absorbing;

#define $Self Absorbing
#define $self absorbing
#include <material/interface.inl>
#undef $Self
#undef $self

#ifndef HOST
#include "absorbing.cc"
#endif // !HOST
