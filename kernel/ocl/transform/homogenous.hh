#pragma once

#include "shift.hh"
#include "rotation.hh"

#define $Self Homogenous3
#define $self hom3
#define $Outer Shift3
#define $outer shf3
#define $Inner Rotation3
#define $inner rot3
#define $elem real3
#include "chain.inl"
#undef $Self
#undef $self
#undef $Outer
#undef $outer
#undef $Inner
#undef $inner
#undef $elem
