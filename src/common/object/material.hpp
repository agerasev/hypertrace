#pragma once

#include "type.hpp"


class Material : public Type {
public:
    struct Instance : public Type::Instance {};
};
class SurfaceMaterial : public Material {};
class VolumeMaterial : public Material {};
