#pragma once

#include "macros.hh"
#include "geometry.hh"

#define $Geo Eu
#define $geo eu
#include "ray.inl"
#undef $Geo
#undef $geo

#define $Geo Hy
#define $geo hy
#include "ray.inl"
#undef $Geo
#undef $geo
