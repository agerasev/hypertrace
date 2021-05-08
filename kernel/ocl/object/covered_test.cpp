#include <types.hh>
#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <shape/eu/sphere.hh>
#include <material/basic.hh>
#include <render/light/eu.hh>

typedef void CoveredTest;

__global SphereEu *covered_test__shape__gc(__global const CoveredTest *self) {
    return NULL;
}

__global Lambertian *covered_test__material__gc(__global const CoveredTest *self) {
    return NULL;
}


#define $Self CoveredTest
#define $self covered_test
#define $Geo Eu
#define $geo eu
#define $Shape SphereEu
#define $shape sphere_eu
#define $Material Lambertian
#define $material lambertian
#include "covered.inl"
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $Shape
#undef $shape
