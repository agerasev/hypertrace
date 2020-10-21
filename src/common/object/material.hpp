#pragma once

#include <host/dyntype/type.hpp>


class Material : public dyn::Type {
public:
    struct Instance : public dyn::Type::Instance {};

    virtual std::string prefix() = 0;
};

class SurfaceMaterial : public Material {};
class VolumeMaterial : public Material {};
