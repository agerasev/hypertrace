#include <types.hh>
#include <geometry/euclidean.hh>
#include "eu/sphere.hh"

struct MappedSphere {
    Shift3 map;
    //SphereEu shape;
};

__global const SphereEu *mapped_sphere__shape__gc(__global const MappedSphere *self) {
    return NULL;
}

#define $Self MappedSphere
#define $self mapped_sphere
#define $Geo Eu
#define $geo eu
#define $Map Shift3
#define $map shf3
#define $Shape SphereEu
#define $shape sphere_eu
#include <geometry/ray_map.inl>
#include "mapped.inl"
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $Shape
#undef $shape
