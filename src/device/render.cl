#define OPENCL_DEVICE
#define OPENCL_INTEROP

#include <algebra/quaternion.h>
#include <algebra/moebius.h>
#include <geometry/surface.h>
#include <hash.h>

#define EPS 1e-6


__kernel void render(
	__global uchar *screen,
	int width, int height,
	MoebiusPacked view,
	__global SurfacePacked *surfaces,
	const int surface_count
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

	int mi = -1;
	float ml = -1.0f;
	quaternion mp, mn;
	for (int i = 0; i < surface_count; ++i) {
		SurfacePacked sp = surfaces[i];
		Surface s;
		surface_unpack(&s, &sp);

		quaternion p, n;
		float l = surface_hit(&s, &m, &p, &n);
		if (l > 0.0f && (l < ml || mi < 0)) {
			mi = i;
			ml = l;
			mp = p;
			mn = n;
		}
	}

	const float br = 0.05f;
	if (mi == 0) {
		color = (float3)(1.0f, 1.0f, 1.0f);
		quaternion k;
		quaternion f = fract(mp, &k);
		
		int hs = 0;
		hash(&hs, (int)k.x);
		hash(&hs, (int)k.y);

		if (f.x < br || f.x > 1.0f - br || f.y < br || f.y > 1.0f - br) {
			color = (float3)(0.6f, 0.6f, 1.0f);
		} else {
			color = (float3)(0.3f) + 0.2f*(float3)(hs & 1, (hs>>1) & 1, (hs>>2) & 1);
		}
	} else if(mi == 1) {
		color = (float3)(0.5f + 0.5*(mp.x > 0), 0.5f + 0.5*(mp.y > 0), 0.5f);
	}

	uchar4 pix = (uchar4)(convert_uchar3(255*clamp(color, 0.0f, 1.0f)), 0xff);
	vstore4(pix, idx, screen);
}
