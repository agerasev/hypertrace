__kernel void render(
	__global float *screen,
	__global uchar *image,
	int width, int height,
    int sample_no
) {
	int idx = get_global_id(0);

    float3 color = (float3)(1.0f, 0.5f, 0.0f);

	float3 avg_color = (color + vload3(idx, screen)*sample_no)/(sample_no + 1);
	vstore3(avg_color, idx, screen);

	float3 out_color = clamp(avg_color, 0.0f, 1.0f);

	uchar4 pix = (uchar4)(convert_uchar3(0xff*out_color), 0xff);
	vstore4(pix, idx, image);
}
