#define OPENCL_DEVICE
#define OPENCL_INTEROP

#include <types.h>

#include <algebra/quaternion.h>
#include <algebra/moebius.h>

#include <color.h>
#include <ray.h>
#include <object.h>

#define EPS 1e-6


__kernel void render(
	__global uchar *screen,
	int width, int height,
	MoebiusPacked view,
	__global ObjectPacked *objects,
	const int object_count
) {
	int idx = get_global_id(0);

	quaternion v = q_new(
		((real)(idx % width) - 0.5f*(width - 1))/height,
		((real)(idx / width) - 0.5f*(height - 1))/height,
		1.0f, 0.0f
	);

	float3 out_color = (float3)(0.0f);

	Moebius u;
	moebius_unpack(&u, &view);
	quaternion p = q_new((real)0, (real)0, (real)1, (real)0);

	Ray ray;
	ray.direction = q_norm(moebius_deriv(&u, p, v));
	ray.start = moebius_apply(&u, p);
	ray.intensity = color_new_gray(1.0f);

	int prev = -1;
	for (int k = 0; k < 2; ++k) {
		int mi = -1;
		float ml = -1.0f;
		HitInfo minfo;
		for (int i = 0; i < object_count; ++i) {
			if (prev == i) {
				continue;
			}
			Object obj = object_unpack_copy(objects[i]);
			HitInfo info;
			real l = object_hit(&obj, &info, &ray);
			if (l > 0.0f && (l < ml || mi < 0)) {
				mi = i;
				ml = l;
				minfo = info;
			}
		}

		if (mi >= 0) {
			Ray new_ray;
			Object obj = object_unpack_copy(objects[mi]);
			if (object_emit(&obj, &minfo, &ray, &new_ray)) {
				ray = new_ray;
			}
		} else {
			out_color += (float3)(1.0f)*ray.intensity;
			break;
		}
	}

	uchar4 pix = (uchar4)(convert_uchar3(255*clamp(out_color, 0.0f, 1.0f)), 0xff);
	vstore4(pix, idx, screen);
}
