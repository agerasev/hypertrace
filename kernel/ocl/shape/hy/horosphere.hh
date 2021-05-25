#pragma once

#include <geometry/hyperbolic.hh>
#include <render/light/hy.hh>

typedef void Horosphere;

#define $Self Horosphere
#define $self horosphere
#define $Geo Hy
#define $geo hy
#include <shape/interface.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo

#ifndef HOST
#include "horosphere.cc"
#endif // !HOST
