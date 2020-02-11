#define OPENCL

#include <common/moebius.h>


__kernel void display(__global uchar *screen, int width, int height) {
	int p = get_global_id(0);

	float3 color = (float3)(0.0f);

	float3 pos = (float3)(0.0f);
	float3 dir;
	dir.x = ((float)(p % width) - 0.5f*(width - 1))/height;
	dir.y = ((float)(p / width) - 0.5f*(height - 1))/height;
	dir.z = -1.0f;
	dir = normalize(dir);

	float3 spos = (float3)(0.0f, 0.0f, -5.0f);
	float srad = 1.0f;
	if (length(dot(spos - pos, dir)*dir - spos) < srad) {
		color = (float3)(1.0f, 1.0f, 1.0f);
	}

	uchar4 pix = (uchar4)(convert_uchar3(255*clamp(color, 0.0f, 1.0f)), 0xff);
	vstore4(pix, p, screen);
}
