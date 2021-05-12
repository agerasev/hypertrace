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

typedef struct {
    Vector_CoveredTestEu objects;
} ObjectVectorTest;

#define $Self ObjectVectorTest
#define $self object_vector_test
#define $Geo Eu
#define $geo eu
#define $Object CoveredTestEu
#define $object covered_test_eu
#include <object/vector.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Object
#undef $object
