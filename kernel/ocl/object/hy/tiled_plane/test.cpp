#include <material/basic/lambertian.hh>
#include <shape/hy/plane.hh>
#include <render/light/hy.hh>

typedef struct ColoredLambertian {
    color3 color;
    // Lambertian inner;
} ColoredLambertian;

_ALLOW_MULTIPLE_DEFINITIONS_
__global const Lambertian *colored_lambertian__inner__gc(__global const ColoredLambertian *self) {
    return NULL;
}

#define $Self ColoredLambertian
#define $self colored_lambertian
#define $Material Lambertian
#define $material lambertian
#include <material/colored.inl>
#undef $Self
#undef $self
#undef $Material
#undef $material

#define $Self TiledPlaneHy
#define $self tiled_plane_hy
#define $Cache TiledPlaneHyCache
#define $cache tiled_plane_hy_cache
#define $Material ColoredLambertian
#define $material colored_lambertian
#define $material_count 3
#define $tiling_type 2

typedef struct $2($Self,_Array) {
    $Material elements[$material_count];
} $2($Self,_Array);

typedef struct $Self {
    $2($Self,_Array) materials;
    $Material border_material;
    real cell_size;
    real border_width;
} $Self;

typedef struct $Cache {
    HyDir normal;
} $Cache;

#include "impl.inl"

#undef $Self
#undef $self
#undef $Material
#undef $material
#undef $material_count
#undef $tiling_type
