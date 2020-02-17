#define OPENCL_DEVICE
#define OPENCL_INTEROP

#include <algebra/quaternion.h>
#include <algebra/moebius.h>
#include <geometry/surface.h>

#define EPS 1e-6


__kernel void render(
	__global uchar *screen,
	int width, int height,
	MoebiusPacked view,
	SurfacePacked surface
) {
	int idx = get_global_id(0);

	quaternion v = q_new(
		((real)(idx % width) - 0.5f*(width - 1))/height,
		((real)(idx / width) - 0.5f*(height - 1))/height,
		1.0f, 0.0f
	);

	float3 color = (float3)(0.0f);

	Moebius c, u, m;
	moebius_new_lookat(&c, v);
	moebius_unpack(&u, &view);
	moebius_chain(&m, &c, &u);

	Surface s;
	surface_unpack(&s, &surface);

	quaternion g, n;
	if (surface_hit(&s, &m, &g, &n)) {
		quaternion f = fract(2.0f*g, &g);
		float br = 0.1f;
		if (f.x < br || f.x > 1.0f - br || f.y < br || f.y > 1.0f - br) {
			color = (float3)(0.75f, 0.75f, 1.0f);
		} else {
			color = (float3)(0.5f, 0.5f, 0.5f);
		}
	}

	uchar4 pix = (uchar4)(convert_uchar3(255*clamp(color, 0.0f, 1.0f)), 0xff);
	vstore4(pix, idx, screen);
}
