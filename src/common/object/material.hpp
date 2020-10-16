#pragma once

#include "type.hpp"


class Material : public Type {
public:
    struct Instance : public Type::Instance {};

    virtual std::string prefix() = 0;
};

class SurfaceMaterial : public Material {};
class VolumeMaterial : public Material {};
