#define OPENCL_DEVICE
#define OPENCL_INTEROP

#include <moebius.h>


#define EPS 1e-6

__kernel void display(
	__global uchar *screen,
	int width, int height,
	MoebiusPacked view,
	quaternion_packed p, quaternion_packed d, real mu
) {
	int idx = get_global_id(0);

	float2 v = (float2)(
		(float)(idx % width) - 0.5f*(width - 1),
		(float)(idx / width) - 0.5f*(height - 1)
	)/height;

	float3 color = (float3)(0.0f);

	// We look at the top (along the z axis)
	real phi = -atan2(v.y, v.x);
	real theta = -atan2(length(v), 1.0f);

	Moebius a, b, c;
	moebius_new_zrotate(&a, phi);
	moebius_new_xcoil(&b, theta);
	moebius_chain(&c, &b, &a);

	Moebius u, m;
	moebius_unpack(&u, &view);
	moebius_chain(&m, &c, &u);

	d = q_norm(moebius_deriv(&m, p, d));
	p = moebius_apply(&m, p);

	real h = 0.0f;
	if (fabs(mu + d.z) > EPS) {
		real r = p.z/(mu + d.z);
		quaternion c = p - r*d;
		real rd = r*r - c.x*c.x - c.y*c.y;
		if (rd > 0.0f) {
			rd = sqrt(rd);
			h = c.z - rd;
			if (h < 1.0f) {
				h = c.z + rd;
			}
		}
	} else if(fabs(d.z) > EPS) {
		h = dot(p, d)/d.z;
	}

	if (h > 1.0f) {
		Moebius rm;
		quaternion g = q_new(0.0f, 0.0f, h, 0.0f);
		moebius_inverse(&rm, &m);
		g = moebius_apply(&rm, g);
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
