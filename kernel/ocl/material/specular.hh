#pragma once

typedef void Specular;

#define $Self Specular
#define $self specular
#include <material/interface.inl>
#undef $Self
#undef $self

#ifndef HOST
#include "specular.cc"
#endif // !HOST
