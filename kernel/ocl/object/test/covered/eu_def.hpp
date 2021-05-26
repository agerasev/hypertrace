#pragma once

#include <types.hh>
#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <shape/eu/sphere.hh>
#include <material/lambertian.hh>
#include <render/light/eu.hh>

typedef void CoveredTestEu;

typedef struct CoveredTestEuCache {
    EuDir normal;
} CoveredTestEuCache;

_ALLOW_MULTIPLE_DEFINITIONS_
__global const Sphere *covered_test_eu__shape__gc(__global const CoveredTestEu *self) {
    return NULL;
}

_ALLOW_MULTIPLE_DEFINITIONS_
__global const Lambertian *covered_test_eu__material__gc(__global const CoveredTestEu *self) {
    return NULL;
}


#define $Self CoveredTestEu
#define $self covered_test_eu
#define $Geo Eu
#define $geo eu
#define $Shape Sphere
#define $shape sphere_eu
#define $Material Lambertian
#define $material lambertian
#define $Cache CoveredTestEuCache
#define $cache covered_test_eu_cache
#include <object/covered.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Shape
#undef $shape
#undef $Material
#undef $material
#undef $Cache
#undef $cache
