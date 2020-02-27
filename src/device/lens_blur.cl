#pragma once

#include <config.cl>

#include <types.hh>
#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/hyperbolic/ray.hh>
#include <random.hh>


HyRay draw_from_lens(
    Rng *rng,
    quaternion v,
    real focal_length, real lens_radius
) {
    quaternion f = mo_apply(mo_chain(
		mo_inverse(hy_look_to(v)),
		hy_zshift(focal_length)
	), QJ);

	// FIXME: Draw values from uniform distribution on hyperbolic disk
	//        instead of Euclidean one.
	real r = lens_radius*sqrt(rand_uniform(rng));
	real phi = 2*PI*rand_uniform(rng);
	Moebius m = mo_chain(hy_zrotate(phi), hy_xshift(r));
	v = mo_deriv(
        mo_inverse(hy_look_at(mo_apply(mo_inverse(m), f))),
        QJ, QJ
    );

    HyRay ray;
	ray.direction = normalize(mo_deriv(m, QJ, v));
	ray.start = mo_apply(m, QJ);

    return ray;
}
