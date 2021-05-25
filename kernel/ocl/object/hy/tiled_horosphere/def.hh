#pragma once

#include <types.hh>
#include <algebra/quaternion.hh>

typedef uint HorosphereTiling;

#define HOROSPHERE_TILING_UNIFORM   0
#define HOROSPHERE_TILING_SQUARE    1
#define HOROSPHERE_TILING_HEXAGONAL 2

typedef struct TiledHorosphereCache {
   quat normal; 
} TiledHorosphereCache;
