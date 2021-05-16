#include <types.hh>
#include <geometry/euclidean.hh>
#include "eu/sphere.hh"

struct MappedSphere {
    Shift3 map;
    //Sphere inner;
};

__global const Sphere *mapped__inner__gc(__global const MappedSphere *self) {
    return NULL;
}

#define $Self MappedSphere
#define $self mapped_sphere
#define $self_data mapped
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
#undef $self_data
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $Shape
#undef $shape
