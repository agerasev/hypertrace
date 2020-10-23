#pragma once

#include <host/dyntype/type.hpp>


class Material : public dyn::Type {
public:
    class Instance : public dyn::Type::Instance {
    public:
        virtual Material *_type() const override = 0;
        inline rstd::Box<Material> type() const { return rstd::Box<Material>::_from_raw(_type()); }
    };

    virtual Material *_clone() const override = 0;
    rstd::Box<Material> clone() const { return rstd::Box<Material>::_from_raw(_clone()); }

    virtual Instance *_load(const uchar *src) const override = 0;
    rstd::Box<Instance> load(const uchar *src) const { return rstd::Box<Instance>::_from_raw(_load(src)); }

    virtual std::string prefix() const = 0;
};

class SurfaceMaterial : public Material {};
class VolumeMaterial : public Material {};
