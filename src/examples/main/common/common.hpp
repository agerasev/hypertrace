#pragma once

#include <object/variant.hpp>
#include <object/hyperbolic/plane.hpp>
#include <object/hyperbolic/horosphere.hpp>
#include <object/material.hpp>
#include <object/combination.hpp>
#include <object/covered.hpp>
#include <object/mapped.hpp>


typedef Combination<
    Specular,
    Colored<Lambertian>
> MyBaseMaterial;

typedef Combination<
    Transparent,
    Emissive<MyBaseMaterial>
> MyMaterial;

MyMaterial make_material(
    float3 diffuse,
    real specularity=0,
    real transparency=0,
    float3 emission=float3(0)
) {
    return MyMaterial(
        make_pair(transparency, Transparent()),
        make_pair(
            1 - transparency,
            Emissive<MyBaseMaterial>(
                emission,
                MyBaseMaterial(
                    make_pair(specularity, Specular()),
                    make_pair(1 - specularity, Colored<Lambertian>(diffuse))
                )
            )
        )
    );
}

typedef hy::TiledPlane<MyMaterial, 2> MyPlane;
typedef hy::TiledHorosphere<MyMaterial, 4> MyHorosphere;

typedef Array<MyMaterial, 2> MyPlaneMaterials;
typedef Array<MyMaterial, 4> MyHorosphereMaterials;

typedef Mapped<VariantObject<MyPlane, MyHorosphere>> MyObject;


MyObject make_plane(
    Moebius map,
    MyPlane::Tiling tiling,
    const MyPlaneMaterials &mats,
    real bw = 0,
    const MyMaterial &bm=make_material(float3(0))
) {
    return MyObject(
        map,
        VariantObject<MyPlane, MyHorosphere>::init<0>(MyPlane(
            tiling, mats, bm, bw
        ))
    );
}
MyObject make_horosphere(
    Moebius map,
    MyHorosphere::Tiling tiling,
    const MyHorosphereMaterials &mats,
    real cs = 1,
    real bw = 0,
    const MyMaterial &bm=make_material(float3(0))
) {
    return MyObject(
        map,
        VariantObject<MyPlane, MyHorosphere>::init<1>(MyHorosphere(
            tiling, mats, cs, bm, bw
        ))
    );
}
