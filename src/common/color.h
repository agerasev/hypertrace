#pragma once


#ifdef OPENCL_DEVICE

typedef float3 color;

#ifdef OPENCL_INTEROP
typedef color color_packed;
#endif // OPENCL_INTEROP

#else // OPENCL_DEVICE

#include <CL/cl.h>

typedef struct color color;

#ifdef OPENCL_INTEROP
typedef cl_float3 color_packed;
#endif // OPENCL_INTEROP

struct color {
    float r, g, b;

#ifdef __cplusplus
    color();
    color(float _r, float _g, float _b);
#ifdef OPENCL_INTEROP
    color_packed pack() const;
#endif // OPENCL_INTEROP

#endif // __cplusplus
};

#endif // OPENCL_DEVICE

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

color color_new(float r, float g, float b);
color color_new_gray(float v);

color color_add(color a, color b);
color color_sub(color a, color b);
color color_mulcc(color a, color b);
color color_mulfc(float a, color b);
color color_mulcf(color a, float b);

#ifdef OPENCL_INTEROP
color_packed color_pack(color c);
color color_unpack(color_packed c);
#endif // OPENCL_INTEROP

#ifdef __cplusplus
};
#endif // __cplusplus



#ifdef OPENCL_INTEROP

color_packed color_pack(color c) {
#ifdef OPENCL_DEVICE
    return (color_packed)c;
#else // OPENCL_DEVICE
    color_packed p;
    p.s[0] = c.r;
    p.s[1] = c.g;
    p.s[2] = c.b;
    return p;
#endif // OPENCL_DEVICE
}

color color_unpack(color_packed p) {
#ifdef OPENCL_DEVICE
    return (color)p;
#else // OPENCL_DEVICE
    color c;
    c.r = p.s[0];
    c.g = p.s[1];
    c.b = p.s[2];
    return c;
#endif // OPENCL_DEVICE
}


color color_new(float r, float g, float b) {
#ifdef OPENCL_DEVICE
    return (color)(r, g, b);
#else // OPENCL_DEVICE
    color c;
    c.r = r;
    c.g = g;
    c.b = b;
    return c;
#endif // OPENCL_DEVICE
}

color color_new_gray(float v) {
#ifdef OPENCL_DEVICE
    return (color)(v);
#else // OPENCL_DEVICE
    color c;
    c.r = v;
    c.g = v;
    c.b = v;
    return c;
#endif // OPENCL_DEVICE
}

color color_add(color a, color b) {
#ifdef OPENCL_DEVICE
    return a + b;
#else // OPENCL_DEVICE
    color c;
    c.r = a.r + b.r;
    c.g = a.g + b.g;
    c.b = a.b + b.b;
    return c;
#endif // OPENCL_DEVICE
}
color color_sub(color a, color b) {
#ifdef OPENCL_DEVICE
    return a - b;
#else // OPENCL_DEVICE
    color c;
    c.r = a.r - b.r;
    c.g = a.g - b.g;
    c.b = a.b - b.b;
    return c;
#endif // OPENCL_DEVICE
}

color color_mulcc(color a, color b) {
#ifdef OPENCL_DEVICE
    return a*b;
#else // OPENCL_DEVICE
    color c;
    c.r = a.r*b.r;
    c.g = a.g*b.g;
    c.b = a.b*b.b;
    return c;
#endif // OPENCL_DEVICE
}
color color_mulfc(float a, color b) {
#ifdef OPENCL_DEVICE
    return a*b;
#else // OPENCL_DEVICE
    color c;
    c.r = a*b.r;
    c.g = a*b.g;
    c.b = a*b.b;
    return c;
#endif // OPENCL_DEVICE
}
color color_mulcf(color a, float b) {
    return color_mulfc(b, a);
}

#endif // OPENCL_INTEROP

#ifdef __cplusplus

color::color() = default;
color::color(float _r, float _g, float _b) {
    r = _r;
    g = _g;
    b = _b;
}

#ifdef OPENCL_INTEROP
color_packed color::pack() const {
    return color_pack(*this);
}
#endif // OPENCL_INTEROP

#endif // __cplusplus
