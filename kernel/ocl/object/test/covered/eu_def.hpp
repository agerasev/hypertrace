#pragma once

#include <types.hh>
#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <shape/eu/sphere.hh>
#include <material/basic.hh>
#include <render/light/eu.hh>

typedef struct {
    SphereEu shape;
    Lambertian material;
} CoveredTestEu;

#define $Self CoveredTestEu
#define $self covered_test_eu
#define $Geo Eu
#define $geo eu
#define $Shape SphereEu
#define $shape sphere_eu
#define $Material Lambertian
#define $material lambertian
#include <object/covered.inl>
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $Shape
#undef $shape
