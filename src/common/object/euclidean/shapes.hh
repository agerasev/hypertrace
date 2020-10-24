#pragma once

#include <common/algebra/vector.hh>
#include <common/geometry/euclidean.hh>
#include <common/render/context.hh>
#include <common/render/light.hh>


typedef void SphereEu;
real sphereeu_detect(__global const SphereEu *shape, Context *context, real3 *normal, LightEu *light);
typedef void CubeEu;
real cubeeu_detect(__global const CubeEu *shape, Context *context, real3 *normal, LightEu *light);


#ifdef HOST

#include <common/object/shape.hpp>

template <typename G>
class Cube;
template <>
class Cube<Eu> final : public dyn::ImplEmptyType<Cube<Eu>, SurfaceShape<Eu>> {
public:
    inline virtual bool repeat_allowed() const override { return true; }
    inline virtual std::string name() const override { return "CubeEu"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/euclidean/shapes.hh"); };
};

template <typename G>
class Sphere;
template <>
class Sphere<Eu> final : public dyn::ImplEmptyType<Sphere<Eu>, SurfaceShape<Eu>> {
public:
    inline virtual bool repeat_allowed() const override { return true; }
    inline virtual std::string name() const override { return "SphereEu"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/euclidean/shapes.hh"); };
};

#endif // HOST


#ifndef HOST
#include "shapes.cc"
#endif // !HOST
