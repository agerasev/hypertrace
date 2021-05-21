#include <types.hh>
#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <material/basic.hh>
#include <render/light/eu.hh>

#include "covered/eu_def.hpp"

#define $Self Vector_CoveredTestEu
#define $self vector__covered_test_eu
#define $Element CoveredTestEu
#define $element covered_test_eu
#define $element_size 0
#include <container/vector.inl>
#undef $Self
#undef $self
#undef $Element
#undef $element
#undef $element_size

typedef struct ObjectVectorCache {
    usize index;
    CoveredTestEuCache inner;
} ObjectVectorCache;

typedef Vector_CoveredTestEu ObjectVector_CoveredTestEu;

#define $Self ObjectVector_CoveredTestEu
#define $self object_vector__covered_test_eu
#define $Base Vector_CoveredTestEu
#define $base vector__covered_test_eu
#define $Cache ObjectVectorCache
#define $cache object_vector_cache
#define $Geo Eu
#define $geo eu
#define $Object CoveredTestEu
#define $object covered_test_eu
#define $ObjectCache CoveredTestEuCache
#define $object_cache covered_test_eu_cache
#include <object/vector.inl>
#undef $Self
#undef $self
#undef $Base
#undef $base
#undef $Cache
#undef $cache
#undef $Geo
#undef $geo
#undef $Object
#undef $object
#undef $ObjectCache
#undef $object_cache
