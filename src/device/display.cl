#define OPENCL

#include <common/moebius.h>


__kernel void display(__global uchar *screen, int width, int height, float t) {
	const float3 FREQ[2] = {
		(float3)(3.0f, 4.0f, 5.0f),
		(float3)(6.0f, 7.0f, 8.0f)
	};
	int p = get_global_id(0);

	float x = (float)(p % width)/(width - 1);
	float y = (float)(p / width)/(height - 1);

	uchar4 pix = (uchar4)(convert_uchar3(255*clamp(
		x*sin(FREQ[0]*t) + y*sin(FREQ[1]*t),
		0.0f, 1.0f
	)), 0xff);

	vstore4(pix, p, screen);
}
