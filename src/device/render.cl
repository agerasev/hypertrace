#include <config.cl>

#include <types.hh>
#include <random.hh>

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/hyperbolic/ray.hh>

#include <object.hh>
#include <view.hh>

#include <lens_blur.cl>


__kernel void render(
	__global float *screen,
	__global uchar *image,
	int width, int height,
	int sample_no,
	__global uint *seeds,
	ViewPk view_pk,
	__global ObjectPk *objects,
	const int object_count
) {
	int idx = get_global_id(0);
	Rng rng;
	rand_init(&rng, seeds[idx]);

	View view;
	unpack_view(&view, &view_pk);

	quaternion v = q_new(
		((real)(idx % width) - 0.5f*width + rand_uniform(&rng))/height,
		((real)(idx / width) - 0.5f*height + rand_uniform(&rng))/height,
		view.field_of_view, 0.0f
	);

	float3 color = (float3)(0.0f);

	Moebius u = view.position;

#ifdef MOTION_BLUR
	Moebius w = view.motion;
	u = mo_chain(u, mo_pow(w, rand_uniform(&rng)));
#endif // MOTION_BLUR

	HyRay ray = hyray_init();
#ifdef LENS_BLUR
	ray = draw_from_lens(&rng, v, view.focal_length, view.lens_radius);
#endif // LENS_BLUR
	ray = hyray_map(u, ray);

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


#include <source.cl>
