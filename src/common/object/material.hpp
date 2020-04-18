#pragma once

#include <types.h>
#include <render/context.hpp>


struct BounceParams {
    // in
    real3 normal;
    real3 enter_dir;
    // out
    real3 &exit_dir;
    float3 &light;
    float3 &emission;
};

class Material {};

class Void : public Material {
public:
    bool bounce(Context &context, BounceParams &params);
};
class Transparent : public Material {
public:
    bool bounce(Context &context, BounceParams &params);
};

class Void : public Material {
public:
    bool bounce(Context &context, BounceParams &params);
};
class Specular : public Material {
public:
    bool bounce(Context &context, BounceParams &params);
};
class Lambertian : public Material {
public:
    bool bounce(Context &context, BounceParams &params);
};

template <typename M>
class Colored {
public:
    float3 color;
    bool bounce(Context &context, BounceParams &params);
};
template <typename M>
class Emission {
public:
    float3 color;
    bool bounce(Context &context, BounceParams &params);
};
