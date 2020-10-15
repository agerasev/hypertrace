#pragma once

#include <common/geometry/geometry.hpp>


// Trait for all shapes
template <typename T>
struct Shape {
    //typedef G Geo;
    // Allow repeated collisions or not
    //static const bool repeated = false;

    //static std::string name();
    //static std::string source();
};

/*
template <typename T>
struct SurfaceShape : public Shape<T> {};

template <typename T>
struct VolumeShape : public Shape<T> {
    static const bool repeated = true;
};
*/
