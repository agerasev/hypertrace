#include "types.hh"

#ifndef OPENCL

#ifdef OPENCL_INTEROP

#include <CL/cl.h>


float2_pk pack_float2(float2 v) {
    return (float2_pk) { .s = {v[0], v[1]} };
}
float3_pk pack_float3(float3 v) {
    return (float3_pk) { .s = {v[0], v[1], v[2]} };
}
float4_pk pack_float4(float4 v) {
    return (float4_pk) { .s = {v[0], v[1], v[2], v[3]} };
}
float2 unpack_float2(float2_pk v) {
    return float2::load(v.s);
}
float3 unpack_float3(float3_pk v) {
    return float3::load(v.s);
}
float4 unpack_float4(float2_pk v) {
    return float4::load(v.s);
}

float2_pk pack_double2(double2 v) {
    return (float2_pk) { .s = {(cl_float)v[0], (cl_float)v[1]} };
}
float3_pk pack_double3(double3 v) {
    return (float3_pk) { .s = {(cl_float)v[0], (cl_float)v[1], (cl_float)v[2]} };
}
float4_pk pack_double4(double4 v) {
    return (float4_pk) { .s = {(cl_float)v[0], (cl_float)v[1], (cl_float)v[2], (cl_float)v[3]} };
}
double2 unpack_double2(float2_pk v) {
    return float2::load(v.s).cast<double>();
}
double3 unpack_double3(float3_pk v) {
    return float3::load(v.s).cast<double>();
}
double4 unpack_double4(float2_pk v) {
    return float4::load(v.s).cast<double>();
}

#endif // OPENCL_INTEROP

#endif // OPENCL
