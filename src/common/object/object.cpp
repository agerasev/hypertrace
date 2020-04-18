#include "object.hpp"

#include <geometry/hyperbolic/plane.hpp>
#include <geometry/hyperbolic/horosphere.hpp>

using namespace random;
using namespace hyperbolic;

real Object::hit(
    ObjectHit *cache,
    Rng *rng, PathInfo *path,
    HyRay ray
) {
    
}

bool object_bounce(
    const Object *object, const ObjectHit *cache,
    Rng *rng, PathInfo *path,
    HyRay *ray,
    float3 *light, float3 *emission
) {
    
}
