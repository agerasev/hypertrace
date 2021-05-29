#pragma once

#include <algebra/real.hh>
#include "interface.hh"

typedef struct Refractive {
    real index;
} Refractive;

define_material_interface(Refractive, refractive)

#ifndef HOST
#include "refractive.cc"
#endif // !HOST
