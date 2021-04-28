#pragma once

#include <algebra/vector.hh>
#include <transform/homogenous.hh>

#define $Geo Eu
#define $geo eu
#define $Map Homogenous3
#define $map hom3
#define $pos real3
#define $dir real3
#include "interface.inl"
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $pos
#undef $dir

EuMap eu_shift(EuDir pos);
EuMap eu_rotate(EuDir axis, real phi);

#ifndef HOST
#include "euclidean.cc"
#endif // !HOST
