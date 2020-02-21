#pragma once

#include <algebra/quaternion.hh>

typedef struct Ray {
    quaternion start;
    quaternion direction;
    float3 intensity;
} Ray;
