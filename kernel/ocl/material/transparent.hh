#pragma once

#include "interface.hh"

typedef void Transparent;

define_material_interface(Transparent, transparent)

#ifndef HOST
#include "transparent.cc"
#endif // !HOST
