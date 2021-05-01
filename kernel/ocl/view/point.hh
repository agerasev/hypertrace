#pragma once

#include <macros.hh>
#include <geometry/geometry.hh>
#include <geometry/ray.hh>

#define $Geo Eu
#define $geo eu
#include "point.inl"
#undef $Geo
#undef $geo

#define $Geo Hy
#define $geo hy
#include "point.inl"
#undef $Geo
#undef $geo
