#pragma once

#include <geometry/euclidean.hh>
#include <render/light/eu.hh>
#include <shape/primitive.hh>

typedef Plane PlaneEu;

#define $Self PlaneEu
#define $self plane_eu
#define $Geo Eu
#define $geo eu
#include <shape/interface.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo

#ifndef HOST
#include "plane.cc"
#endif // !HOST
