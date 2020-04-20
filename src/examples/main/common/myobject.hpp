#pragma once

#include <object/mapped.hpp>
#include <object/variant.hpp>
#include <object/covered.hpp>
#include <object/hyperbolic/plane.hpp>
#include <object/hyperbolic/horosphere.hpp>
#include <object/material.hpp>

typedef VariantShape<hy::Plane, hy::Horosphere> MyShape;
typedef Colored<Lambertian> MyMaterial;
typedef Covered<MyShape, MyMaterial> MyObject;
