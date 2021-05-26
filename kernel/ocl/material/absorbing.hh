#pragma once

#include "interface.hh"

typedef void Absorbing;

define_material_interface(Absorbing, absorbing)

#ifndef HOST
#include "absorbing.cc"
#endif // !HOST
