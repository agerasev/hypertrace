#include <types.hh>
#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <shape/eu/sphere.hh>
#include <render/light/eu.hh>

#define $Self Vector_Sphere
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

#define $Self ShapeVector_SphereEu
#define $self shape_vector__sphere_eu
#define $Base Vector_Sphere
#define $base vector__sphere_eu
#define $Geo Eu
#define $geo eu
#define $Shape SphereEu
#define $shape sphere_eu
#include <shape/vector.inl>
#undef $Self
#undef $self
#undef $Base
#undef $base
#undef $Geo
#undef $geo
#undef $Shape
#undef $shape
