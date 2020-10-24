#pragma once

#include <host/dyntype/type.hpp>


class Material : public dyn::Type {
public:
    class Instance : public dyn::Type::Instance {
    public:
        virtual Material *_type() const override = 0;
        inline rs::Box<Material> type() const { return rs::Box<Material>::_from_raw(_type()); }
    };

    virtual Material *_clone() const override = 0;
    rs::Box<Material> clone() const { return rs::Box<Material>::_from_raw(_clone()); }

    virtual Instance *_load(const uchar *src) const override = 0;
    rs::Box<Instance> load(const uchar *src) const { return rs::Box<Instance>::_from_raw(_load(src)); }

    std::string prefix() const { return rs::to_lower(name()); }
};

class SurfaceMaterial : public Material {};
class VolumeMaterial : public Material {};
