#pragma once

#include <object/variant.hpp>
#include <object/hyperbolic/plane.hpp>
#include <object/hyperbolic/horosphere.hpp>
#include <object/material.hpp>
#include <object/combination.hpp>
#include <object/covered.hpp>
#include <object/mapped.hpp>


typedef VariantShape<hy::Plane, hy::Horosphere> MyShape;
typedef Combination<Specular, Colored<Lambertian>> MyMaterial;
typedef Covered<MyShape, MyMaterial> MyObject;
