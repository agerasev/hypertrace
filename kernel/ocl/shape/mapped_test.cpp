#include <types.hh>
#include <geometry/euclidean.hh>
#include "eu/sphere.hh"

struct Mapped {
    Shift3 map;
    //Sphere inner;
};

_ALLOW_MULTIPLE_DEFINITIONS_
__global const Sphere *mapped__inner__gc(__global const Mapped *self) {
    return NULL;
}

#define $Self ShapeMapped
#define $self shape_mapped
#define $Base Mapped
#define $base mapped
#define $Geo Eu
#define $geo eu
#define $Map Shift3
#define $map shf3
#define $Shape Sphere
#define $shape sphere_eu
#include <geometry/ray_map.inl>
#include "mapped.inl"
#undef $Self
#undef $self
#undef $Base
#undef $base
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $Shape
#undef $shape
