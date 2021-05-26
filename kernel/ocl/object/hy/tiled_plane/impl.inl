#include <types.hh>
#include <math.hh>
#include <object/interface.hh>

#define PLANE_HY_TILING_UNIFORM    0
#define PLANE_HY_TILING_PENTAGONAL 1
#define PLANE_HY_TILING_PENTASTAR  2

#if \
    !defined($Self) || !defined($self) || \
    !defined($Material) || !defined($material) || \
    !defined($material_count) || !defined($tiling_type)
#error "All required macro parameters must be defined."
#endif

define_object_interface(
    $Self, $self,
    $Cache, $cache,
    Hy, hy
)

_ALLOW_MULTIPLE_DEFINITIONS_
real $2($self,_detect)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    LightHy *light
) {
    return plane_hy_detect(
        NULL,
        context,
        &cache->normal,
        light
    );
}

_ALLOW_MULTIPLE_DEFINITIONS_
bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    LightHy *light,
    color3 *emission
) {
    __global const $Material *material = NULL;

#if $tiling_type == PLANE_HY_TILING_PENTAGONAL || \
    $tiling_type == PLANE_HY_TILING_PENTASTAR
        quat p = light->ray.start;

        bool w = false;
        //uint n = 0, b = 1;
        real Q = sqrt(cos(PI / 4 + PI / 5) / cos(PI / 4 - PI / 5));
        real T = sqrt(cos(PI / 4 + PI / 5) * cos(PI / 4 - PI / 5));
        real S = (cos(PI / 4) - sin(PI / 5)) / T;
        real L = T / cos(PI / 4);
        real K = L * (2 * cos(PI / 5) - 1 / cos(PI / 5));
        Q = log((1 + Q) / (1 - Q));
        S = log((1 + S) / (1 - S));

        if (p.x < R0) {
            p.x = -p.x;
            //n |= 1;
            w = !w;
        }
        if (p.y < R0) {
            p.y = -p.y;
            //n |= 2;
            w = !w;
        }
        //b *= 4;

        p = hy_apply_pos(
            hy_chain(hy_xshift(-Q), hy_zrotate(-PI / 4)),
            p
        );
        bool e = false;
        for (int j = 0; j < 5; ++j) {
            bool a[3] = {false, false, false};
            for (int i = 0; i < 3 - e; ++i) {
                real o = 2 * PI * (i - 1) / 5;
                real2 d = make(real2)(cos(o), sin(o));
                a[i] = (dot(d, p.xy) < L);
            }
            a[2] = a[2] || e;
            int s = (int)a[0] + (int)a[1] + (int)a[2];
            //uint bb = b*(6 - 2*e);
            if (s == 3) {
                break;
            } else if (s == 2) {
                int i = (!a[1]) + 2*(!a[2]);
                real o = 2 * PI * (i - 1) / 5;
                p = hy_apply_pos(
                    hy_chain(hy_zrotate(-PI / 5), hy_chain(hy_xshift(-2 * S), hy_zrotate(-o))),
                    p
                );
                //n += b*(2*i + 1);
                e = true;
                w = !w;
            } else {
                int i = a[0];
                real o = PI * (2 * i - 1) / 5;
                p = hy_apply_pos(
                    hy_chain(hy_xshift(-2 * Q), hy_zrotate(-o)),
                    p
                );
                //n += b*(2*(i + 1));
                e = false;
            }
            //b = bb;
        }
        
        const real br = self->border_width;
        bool bh = 0;
        for (int i = 0; i < 5; ++i) {
            real o = 2 * PI * i / 5;
            real2 d = make(real2)(cos(o), sin(o));
            bh = bh || (dot(d, p.xy) > (L - br * p.z));
#if $tiling_type == PLANE_HY_TILING_PENTASTAR
            real ps = K + dot(d, p.xy);
            w = w ^ (ps < 0);
            bh = bh || (fabs(ps) < br * p.z);
#endif
        }
        if (bh == 0) {
            if (w || $material_count < 2) {
                material = &self->materials.elements[0];
            } else {
                material = &self->materials.elements[1];
            }
        } else {
            material = &self->border_material;
        }
#else
        material = &self->materials.elements[0];
#endif

    LightLocal ll = light_hy_to_local(light);
    bool ret = $2($material,_interact)(
        material,
        context,
        hy_dir_to_local(light->ray.start, cache->normal),
        &ll,
        emission
    );
    light_hy_update_local(light, ll);
    return ret;
}
