#pragma once

#include <common/algebra/vector.hh>
#include <common/geometry/euclidean.hh>
#include <common/render/context.hh>
#include <common/render/light.hh>


real sphereeu_detect(Context *context, real3 *normal, LightEu *light);
real cubeeu_detect(Context *context, real3 *normal, LightEu *light);


#ifdef HOST

#include <common/object/shape.hpp>

template <typename G>
class Cube;
template <>
class Cube<Eu> final : public dyn::ImplEmptyType<Cube<Eu>, SurfaceShape<Eu>> {
public:
    inline virtual bool repeat_allowed() const override { return true; }
    inline virtual std::string prefix() const override { return "cubeeu"; };
    inline virtual std::string source() const override { return "#include <common/object/euclidean/shapes.hh>"; };
};

template <typename G>
class Sphere;
template <>
class Sphere<Eu> final : public dyn::ImplEmptyType<Sphere<Eu>, SurfaceShape<Eu>> {
public:
    inline virtual bool repeat_allowed() const override { return true; }
    inline virtual std::string prefix() const override { return "sphereeu"; };
    inline virtual std::string source() const override { return "#include <common/object/euclidean/shapes.hh>"; };
};

#endif // HOST


#ifndef HOST
#include "shapes.cc"
#endif // !HOST
