#pragma once

#include <algebra/quaternion.h>
#include <color.h>

typedef struct Ray {
    quaternion start;
    quaternion direction;
    color intensity;
} Ray;
