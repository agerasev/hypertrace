#pragma once

#include "real.hh"
#include "complex.hh"
#include "quaternion.hh"


typedef complex rotation2;
typedef quaternion rotation3;


rotation2 rot2_new(real angle);
real2 rot2_apply(rotation2 rot, real2 pos);
rotation2 rot2_chain(rotation2 a, rotation2 b);

rotation3 rot3_new(real3 axis, real angle);
real3 rot3_apply(rotation3 rot, real3 pos);
rotation3 rot3_chain(rotation3 a, rotation3 b);

rotation2 rot2_look_at(real2 pos);
rotation3 rot3_look_at(real3 pos);
