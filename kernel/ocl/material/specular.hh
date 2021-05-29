#pragma once

#include "interface.hh"

typedef void Specular;

define_material_interface(Specular, specular)

#ifndef HOST
#include "specular.cc"
#endif // !HOST
