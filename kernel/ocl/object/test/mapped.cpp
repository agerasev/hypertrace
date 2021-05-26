#include <types.hh>
#include <geometry/euclidean.hh>
#include "covered/eu_def.hpp"

struct Mapped {
    Shift3 map;
    //CoveredTestEu inner;
};

static
__global const CoveredTestEu *mapped__inner__gc(__global const Mapped *self) {
    return NULL;
}

#define $Self ObjectMapped
#define $self object_mapped
#define $Base Mapped
#define $base mapped
#define $Geo Eu
#define $geo eu
#define $Map Shift3
#define $map shf3
#define $Object CoveredTestEu
#define $object covered_test_eu
#define $Cache CoveredTestEuCache
#define $cache covered_test_eu_cache
#include <geometry/ray_map.inl>
#include <object/mapped.inl>
#undef $Self
#undef $self
#undef $Base
#undef $base
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $Object
#undef $object
#undef $Cache
#undef $cache
