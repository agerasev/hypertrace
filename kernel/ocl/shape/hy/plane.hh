#pragma once

#include <geometry/hyperbolic.hh>
#include <render/light/hy.hh>
#include <shape/primitive.hh>

typedef Plane PlaneHy;

#define $Self PlaneHy
#define $self plane_hy
#define $Geo Hy
#define $geo hy
#include <shape/interface.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo

#ifndef HOST
#include "plane.cc"
#endif // !HOST
