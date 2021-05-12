#include <types.hh>
#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <shape/eu/sphere.hh>
#include <render/light/eu.hh>

#define $Self Vector_SphereEu
#define $self vector__sphere_eu
#define $Element SphereEu
#define $element sphere_eu
#define $element_size 0
#include <container/vector.inl>
#undef $Self
#undef $self
#undef $Element
#undef $element
#undef $element_size

typedef struct {
    Vector_SphereEu shapes;
} ShapeVectorTest;

#define $Self ShapeVectorTest
#define $self object_vector_test
#define $Geo Eu
#define $geo eu
#define $Shape SphereEu
#define $shape sphere_eu
#include <shape/vector.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Shape
#undef $shape
