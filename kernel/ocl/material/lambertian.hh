#pragma once

#include "interface.hh"

typedef void Lambertian;

define_material_interface(Lambertian, lambertian)

#ifndef HOST
#include "lambertian.cc"
#endif // !HOST
