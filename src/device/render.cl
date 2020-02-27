#include <config.cl>

#include <types.hh>
#include <random.hh>

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/hyperbolic/ray.hh>

#include <object.hh>

#include <number.cl>


__kernel void render(
	__global float *screen,
	__global uchar *image,
	int width, int height,
	int sample_no,
	__global uint *seeds,
	MoebiusPk view, MoebiusPk motion,
	__global ObjectPk *objects,
	const int object_count
) {
	int idx = get_global_id(0);
	Rng rng;
	rand_init(&rng, seeds[idx]);

	/*
	uint num = sizeof(ObjectPk);
	int2 pp = (int2)(idx % width, idx / width);
	int ts = 4;
	if (get_number_pixel(num, pp - (int2)(width - 1 - ts, 6*ts), ts)) {
		vstore4((uchar4)((uchar3)(0), 0xff), idx, image);
		return;
	}
	*/

	quaternion v = q_new(
		((real)(idx % width) - 0.5f*width + rand_uniform(&rng))/height,
		((real)(idx / width) - 0.5f*height + rand_uniform(&rng))/height,
		1.0f, 0.0f
	);

	float3 color = (float3)(0.0f);

	Moebius u = mo_unpack(view), w = mo_unpack(motion);

#ifdef MOTION_BLUR
	u = mo_chain(u, mo_pow(w, rand_uniform(&rng)));
#endif // MOTION_BLUR

	HyRay ray;
#ifdef LENS_BLUR
	real lr = 2e-2f; // lens radius
	real fd = 5.0f; // focal distance

	quaternion f = mo_apply(mo_chain(mo_inverse(hy_look_to(v)), hy_zshift(fd)), QJ);
	// FIXME: Draw values from uniform distribution on hyperbolic disk instead of Euclidean one.
	real r = lr*sqrt(rand_uniform(&rng)), phi = 2*PI*rand_uniform(&rng);
	Moebius lm = mo_chain(hy_zrotate(phi), hy_xshift(r));
	v = mo_deriv(mo_inverse(hy_look_at(mo_apply(mo_inverse(lm), f))), QJ, QJ);

	Moebius ulm = mo_chain(u, lm);
	ray.direction = normalize(mo_deriv(ulm, QJ, v));
	ray.start = mo_apply(ulm, QJ);
#else // LENS_BLUR
	ray.direction = normalize(mo_deriv(u, QJ, v));
	ray.start = mo_apply(u, QJ);
#endif // LENS_BLUR

	float3 light = (float3)(1.0f);

	int prev = -1;
	for (int k = 0; k < 3; ++k) {
		int mi = -1;
		float ml = -1.0f;
		ObjectHit mcache;
		for (int i = 0; i < object_count; ++i) {
			if (prev == i) {
				continue;
			}
			ObjectPk obj_pk = objects[i];
			Object obj;
			unpack_object(&obj, &obj_pk);
			
			ObjectHit cache;
			real l = object_hit(&obj, &cache, &rng, ray);
			if (l > (real)0 && (l < ml || mi < 0)) {
				mi = i;
				ml = l;
				mcache = cache;
			}
		}

		prev = mi;
		if (mi >= 0) {
			HyRay new_ray;
			ObjectPk obj_pk = objects[mi];
			Object obj;
			unpack_object(&obj, &obj_pk);
			if (!object_bounce(&obj, &mcache, &rng, &ray, &light, &color)) {
				break;
			}
		} else {
			color += (float3)(1.0f)*light;
			break;
		}
	}

	seeds[idx] = rng.state;

	float3 avg_color = (color + vload3(idx, screen)*sample_no)/(sample_no + 1);
	vstore3(avg_color, idx, screen);

	uchar4 pix = (uchar4)(convert_uchar3(255*clamp(avg_color, 0.0f, 1.0f)), 0xff);
	vstore4(pix, idx, image);
}
