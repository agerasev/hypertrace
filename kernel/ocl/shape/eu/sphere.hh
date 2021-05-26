#pragma once

#include <geometry/euclidean.hh>
#include <render/light/eu.hh>
#include <shape/primitive.hh>

typedef Sphere SphereEu;

#define $Self SphereEu
#define $self sphere_eu
#define $Geo Eu
#define $geo eu
#include <shape/interface.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo

#ifndef HOST
#include "sphere.cc"
#endif // !HOST
