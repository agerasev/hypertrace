#define OPENCL

#include <common/moebius.h>


#define EPS 1e-6

__kernel void display(__global uchar *screen, int width, int height) {
	int idx = get_global_id(0);

	float3 color = (float3)(0.0f);

	float3 pos = (float3)(0.0f);
	float3 dir;
	dir.x = ((float)(idx % width) - 0.5f*(width - 1))/height;
	dir.y = -((float)(idx / width) - 0.5f*(height - 1))/height;
	dir.z = 100.0f;
	dir = normalize(dir);

	moebius a, b, c;

	float cos2f = dir.x/length(dir.xy);
	float cosf = sqrt((1.0f + cos2f)/2.0f);
	float sinf = sqrt((1.0f - cos2f)/2.0f);
	moebius_new(&b, c_new(cosf, sinf), c_new_r(0.0f),  c_new_r(0.0f), c_new(cosf, -sinf));

	float cos2t = dir.z;
	float cost = sqrt((1.0f + cos2t)/2.0f);
	float sint = sqrt((1.0f - cos2t)/2.0f);
	moebius_new(&a, c_new_r(cost), c_new_r(sint), c_new_r(-sint), c_new_r(cost));

	moebius_chain(&c, &a, &b);

	// Horosphere
	float3 p = (float3)(0.0f, 0.0f, 2.0f);
	float3 d = (float3)(0.0f, 0.0f, -1.0f);
	float mu = 1.0f;

	p = moebius_apply_pos(&c, (quaternion)(p, 0.0f)).xyz;
	d = moebius_apply_dir(&c, (quaternion)(p, 0.0f), (quaternion)(d, 0.0f)).xyz;

	float h = 0.0f;
	if (fabs(mu + d.z) > EPS) {
		float r = p.z/(mu + d.z);
		float3 c = p - r*d;
		h = c.z - sqrt(r*r + c.x*c.x + c.y*c.y);
	} else if(fabs(d.z) > EPS) {
		h = dot(p, d)/d.z;
	}

	if (h > 1.0f) {
		color = (float3)(1.0f, 1.0f, 1.0f);
	}

	uchar4 pix = (uchar4)(convert_uchar3(255*clamp(color, 0.0f, 1.0f)), 0xff);
	vstore4(pix, idx, screen);
}
