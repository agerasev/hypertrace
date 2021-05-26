#include <material/lambertian.hh>
#include <shape/hy/horosphere.hh>
#include <render/light/hy.hh>
#include "def.hh"

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

#define $Self TiledHorosphere
#define $self tiled_horosphere
#define $Material ColoredLambertian
#define $material colored_lambertian
#define $material_count 3
#define $tiling_type 2
#include "impl.inl"
#undef $Self
#undef $self
#undef $Material
#undef $material
#undef $material_count
#undef $tiling_type
