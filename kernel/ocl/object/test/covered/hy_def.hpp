#pragma once

#include <types.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/ray_hy.hh>
#include <shape/hy/horosphere.hh>
#include <material/basic/lambertian.hh>
#include <render/light/hy.hh>

typedef void CoveredTestHy;

typedef struct CoveredTestHyCache {
    EuDir normal;
} CoveredTestHyCache;

_ALLOW_MULTIPLE_DEFINITIONS_
__global const Horosphere *covered_test_hy__shape__gc(__global const CoveredTestHy *self) {
    return NULL;
}

_ALLOW_MULTIPLE_DEFINITIONS_
__global const Lambertian *covered_test_hy__material__gc(__global const CoveredTestHy *self) {
    return NULL;
}


#define $Self CoveredTestHy
#define $self covered_test_hy
#define $Geo Hy
#define $geo hy
#define $Shape Horosphere
#define $shape horosphere
#define $Material Lambertian
#define $material lambertian
#define $Cache CoveredTestHyCache
#define $cache covered_test_hy_cache
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
