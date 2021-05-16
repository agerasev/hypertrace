#pragma once

#include <geometry/euclidean.hh>
#include <render/light/eu.hh>
#include <shape/basic.hh>

#define $Self Cube
#define $self cube_eu
#define $Geo Eu
#define $geo eu
#include <shape/interface.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo

#ifndef HOST
#include "cube.cc"
#endif // !HOST
