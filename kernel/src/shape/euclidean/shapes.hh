#pragma once

#include <algebra/vector.hh>
#include <geometry/euclidean.hh>
#include <render/context.hh>
#include <render/light.hh>


#ifndef DYNTYPE

typedef void SphereEu;
real sphereeu_detect(__global const SphereEu *shape, Context *context, real3 *normal, LightEu *light);

typedef void CubeEu;
real cubeeu_detect(__global const CubeEu *shape, Context *context, real3 *normal, LightEu *light);

#else // DYNTYPE

#include <shape/shape.hpp>

template <typename G>
class Cube;
template <>
class Cube<Eu> final : public dyn::ImplEmptyType<Cube<Eu>, SurfaceShape<Eu>> {
public:
    inline virtual bool repeat_allowed() const override { return true; }
    inline virtual std::string name() const override { return "CubeEu"; };
    inline virtual dyn::Source source() const override { return dyn::Source("shape/euclidean/shapes.hh"); };
};

template <typename G>
class Sphere;
template <>
class Sphere<Eu> final : public dyn::ImplEmptyType<Sphere<Eu>, SurfaceShape<Eu>> {
public:
    inline virtual bool repeat_allowed() const override { return true; }
    inline virtual std::string name() const override { return "SphereEu"; };
    inline virtual dyn::Source source() const override { return dyn::Source("shape/euclidean/shapes.hh"); };
};

#endif // !DYNTYPE


#ifndef HOST
#include "shapes.cc"
#endif // !HOST
