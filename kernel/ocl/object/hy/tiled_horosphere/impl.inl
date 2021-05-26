#include <types.hh>
#include <math.hh>
#include <object/interface.hh>
#include "def.hh"

#if \
    !defined($Self) || !defined($self) || \
    !defined($Material) || !defined($material) || \
    !defined($material_count) || !defined($tiling_type)
#error "All required macro parameters must be defined."
#endif

typedef struct $Self {
    $Material materials[$material_count];
    $Material border_material;
    real cell_size;
    real border_width;
} $Self;

#define $Cache TiledHorosphereCache
#define $cache tiled_horosphere_cache

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
    return horosphere_detect(
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
    int material_no = 0;
    bool border = false;

    real br = self->border_width / self->cell_size;
    real2 g = light->ray.start.xy;

#if $tiling_type == HOROSPHERE_TILING_SQUARE
    real2 k;
    real2 f = fract(g / self->cell_size, &k);

    if (f.x < br || f.x > 1 - br || f.y < br || f.y > 1 - br) {
        border = true;
    } else {
        int hx = rem((int)k.x, 2);
        int hy = rem((int)k.y, 2);
        material_no = 3*hy - 2*hx*hy + hx;
    }

#elif $tiling_type == HOROSPHERE_TILING_HEXAGONAL
    real2 bx = make(real2)(2 / sqrt((real)3), 0);
    real2 by = make(real2)(-1 / sqrt((real)3), 1);
    real s = self->cell_size * sqrt((real)3) / 2;
    real2 h = make(real2)(dot(bx, g), dot(by, g))/s;
    int hx = (int)floor((floor(h.x) - floor(h.y)) / 3);
    int hy = (int)floor((floor(h.x + h.y) - hx) / 2);

    h -= hx * make(real2)(2, -1) + hy * make(real2)(1, 1);
    if (
        fabs(h.x - 1) > 1 - br ||
        fabs(h.y) > 1 - br ||
        fabs(h.x + h.y - 1) > 1 - br
    ) {
        border = true;
    } else {
        material_no = 2*hx + hy;
    }

#else
    material_no = 0;
#endif

    __global const $Material *material;
    if (border) {
        material = &self->border_material;
    } else {
        material = &self->materials[rem(material_no, $material_count)];
    }

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

#undef $Cache
#undef $cache
