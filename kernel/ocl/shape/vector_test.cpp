#include <types.hh>
#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <shape/eu/sphere.hh>
#include <render/light/eu.hh>

#define $Self Vector_Sphere
#define $self vector__sphere_eu
#define $Element Sphere
#define $element sphere_eu
#define $element_size 0
#include <container/vector.inl>
#undef $Self
#undef $self
#undef $Element
#undef $element
#undef $element_size

#define $Self Vector_Sphere
#define $self shape_vector__sphere_eu
#define $self_data vector__sphere_eu
#define $Geo Eu
#define $geo eu
#define $Shape Sphere
#define $shape sphere_eu
#include <shape/vector.inl>
#undef $Self
#undef $self
#undef $self_data
#undef $Geo
#undef $geo
#undef $Shape
#undef $shape
