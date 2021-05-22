#pragma once

typedef void Lambertian;

#define $Self Lambertian
#define $self lambertian
#include <material/interface.inl>
#undef $Self
#undef $self

#ifndef HOST
#include "lambertian.cc"
#endif // !HOST
