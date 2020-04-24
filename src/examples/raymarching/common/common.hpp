#pragma once

#include <object/euclidean/sdf.hpp>
#include <object/material.hpp>
#include <object/combination.hpp>
#include <object/covered.hpp>
#include <object/mapped.hpp>


typedef Combination<
    Colored<Lambertian>,
    Specular
> MyMaterial;

MyMaterial make_material(float3 diffuse, real specularity=0_r) {
    return MyMaterial(
        make_pair(1 - specularity, Colored<Lambertian>(diffuse)),
        make_pair(specularity, Specular())
    );
}

class Sdf {
public:
    real rad;
    real fac;
    real freq;

    real operator()(real3 pos) const {
        real3 cp = math::cos(freq*pos);
        return fac*cp[0]*cp[1]*cp[2] + length(pos) - rad;
    }
    real3 grad(real3 pos) const {
        real3 cp = math::cos(freq*pos);
        real3 sp = math::sin(freq*pos);
        return normalize(
            normalize(pos)
            - fac*freq*real3(
                sp[0]*cp[1]*cp[2],
                cp[0]*sp[1]*cp[2],
                cp[0]*cp[1]*sp[2]
            )
        );
    }
};

#ifdef HOST
template <>
struct ToDevice<Sdf> {
    struct type {
        device::real rad;
        device::real fac;
        device::real freq;
    };
    static type to_device(const Sdf &f) {
        return type{
            .rad = ::to_device(f.rad),
            .fac = ::to_device(f.fac),
            .freq = ::to_device(f.freq),
        };
    }
};
#endif

typedef Mapped<Covered<eu::DistanceFunction<Sdf>, MyMaterial>> MyObject;

MyObject make_object(
    const Affine<real, 3> &map,
    const Sdf &sdf,
    const MyMaterial &mat
) {
    return MyObject(
        map,
        Covered<eu::DistanceFunction<Sdf>, MyMaterial>(
            eu::DistanceFunction<Sdf>(sdf, sdf.rad + sdf.fac, 0.5),
            mat
        )
    );
}
