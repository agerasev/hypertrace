#include <config.cl>

#include <types.hh>
#include <random.hh>

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/hyperbolic/ray.hh>

#include <object.hh>
#include <view.hh>
#include <path.hh>

#include <lens_blur.cl>


__kernel void render(
	__global float *screen,
	__global uchar *image,
	int width, int height,
	int sample_no,
	__global uint *seeds,
	ViewPk view_pk, ViewPk view_prev_pk,
	__global ObjectPk *objects,
	const int object_count
) {
	int idx = get_global_id(0);
	Rng rng;
	rand_init(&rng, seeds[idx]);

	View view = view_unpack(view_pk);

	real time = (real)1;
#ifdef MOTION_BLUR
	{
		time = rand_uniform(&rng);
		view = view_interpolate(view_unpack(view_prev_pk), view, time);
	}
#endif // MOTION_BLUR

	quaternion v = q_new(
		((real)(idx % width) - 0.5f*width + rand_uniform(&rng))/height,
		((real)(idx / width) - 0.5f*height + rand_uniform(&rng))/height,
		view.field_of_view, 0.0f
	);

	float3 color = (float3)(0.0f);

	HyRay ray = hyray_init();
	ray.direction = v;
#ifdef LENS_BLUR
	ray = draw_from_lens(&rng, v, view.focal_length, view.lens_radius);
#endif // LENS_BLUR
	ray = hyray_map(view.position, ray);

	float3 light = (float3)(1.0f);

	PathInfo gpath = {
		.repeat = false,
		.face = false,
		.diffuse = false		
	};
	int prev = -1;
	int diffuse = 0;
	for (int k = 0; k < PATH_MAX_DEPTH; ++k) {
		int mi = -1;
		float ml = -1.0f;
		ObjectHit mcache;
		PathInfo mpath;
		for (int i = 0; i < object_count; ++i) {
			ObjectPk obj_pk = objects[i];
			Object obj;
			unpack_object(&obj, &obj_pk);
			
			ObjectHit cache;
			PathInfo path = gpath;
			path.repeat = (prev == i);
			real l = object_hit(&obj, &cache, &rng, &path, ray);
			if (l > (real)0 && (l < ml || mi < 0)) {
				mi = i;
				ml = l;
				mcache = cache;
				mpath = path;
			}
		}

		if (mi >= 0) {
			HyRay new_ray;
			ObjectPk obj_pk = objects[mi];
			Object obj;
			unpack_object(&obj, &obj_pk);
			if (!object_bounce(
				&obj, &mcache,
				&rng, &mpath,
				&ray,
				&light, &color
			)) {
				break;
			}
		} else {
			color += (float3)(1.0f)*light;
			break;
		}
		prev = mi;
		gpath = mpath;
		if (gpath.diffuse) {
			if (diffuse >= PATH_MAX_DIFFUSE_DEPTH) {
				break;
			}
			diffuse += 1;
		}
	}

	seeds[idx] = rng.state;

	float3 avg_color = (color + vload3(idx, screen)*sample_no)/(sample_no + 1);
	vstore3(avg_color, idx, screen);

	float3 out_color = clamp(avg_color, 0.0f, 1.0f);
#ifdef GAMMA_CORRECTION
	out_color = pow(out_color, 1/GAMMA_VALUE);
#endif // GAMMA_CORRECTION

	uchar4 pix = (uchar4)(convert_uchar3(255*out_color), 0xff);
	vstore4(pix, idx, image);
}


#include <source.cl>
