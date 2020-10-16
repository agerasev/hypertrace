#pragma once

#include "type.hpp"


class Material : public virtual Type {
public:
    struct Instance : public virtual ::Instance {};
};
class SurfaceMaterial : public virtual Material {};
class VolumeMaterial : public virtual Material {};
