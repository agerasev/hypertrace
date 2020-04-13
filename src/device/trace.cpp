#include <types.h>
#include <real.h>
#include <trace.h>

#include <algebra/vector.hpp>
#include <algebra/complex.hpp>

#include <geometry/hyperbolic/ray.hpp>
#include <random.hpp>

using namespace hyperbolic;

bool hit_horosphere(Ray ray) {
    quat p = ray.start, d = ray.direction;
    real dxy = length(d.re());

    if (p[2] < dxy) {
        return false;
    }
    
    real dt = math::sqrt(p[2]*p[2] - dxy*dxy);
    real t = p[2]*d[2] - dt;
    if (t < 0) {
        t += 2*dt;
    }
    if (t < 0) {
        return false;
    }

    t /= dxy*dxy;
    quat h(
        p.re() + d.re()*t,
        1, 0
    );

    return true;
}

void trace(
    float *out_color_ptr,
    const real *pix_pos_ptr, real pix_size,
    uint *rng_seed
) {
    float3 &out_color = *(float3*)out_color_ptr;
    const real2 pix_pos = *(const real2*)pix_pos_ptr;

    random::LCRng rng(*rng_seed);

    out_color = float3(0);

    real2 new_pix_pos = pix_pos + pix_size*(random::uniform<real2>(rng) - real2(0.5_r));
    Ray ray(0.6_j, normalize(quat(new_pix_pos, 1, 0)));

    if (hit_horosphere(ray)) {
        out_color = float3(random::uniform<real3>(rng));
    }

    *rng_seed = rng.next();
}
