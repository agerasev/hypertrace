#include <common/geometry/euclidean.hh>
#include <common/geometry/ray.hh>

#include <common/object/euclidean/shapes.hh>
#include <common/render/view.hh>


__kernel void render(
	__global float *screen,
	__global uchar *image,
	int width, int height,
	int sample_no,
	__global uint *seeds,
	ViewEu view
) {
	int idx = get_global_id(0);
	//Rng rng;
	//rand_init(&rng, seeds[idx]);

	//real time = rand_uniform(&rng);

	real3 v = normalize((real3)(
		((real)(idx % width) - 0.5f*width/* + rand_uniform(&rng)*/)/height,
		((real)(idx / width) - 0.5f*height/* + rand_uniform(&rng)*/)/height,
		view.field_of_view
	));

	RayEu ray;
	ray.start = (real3)(0.0f);
	ray.direction = v;
	ray = rayeu_map(ray, view.position);

	//LightEu light;
	//light.ray = ray;

	float3 color = 0.5f*(ray.direction + 1.0f);

	//seeds[idx] = rng.state;

	float3 avg_color = (color + vload3(idx, screen)*sample_no)/(sample_no + 1);
	vstore3(avg_color, idx, screen);

	float3 out_color = clamp(avg_color, 0.0f, 1.0f);
#ifdef GAMMA_CORRECTION
	out_color = pow(out_color, 1/GAMMA_VALUE);
#endif // GAMMA_CORRECTION

	uchar4 pix = (uchar4)(convert_uchar3(255*out_color), 0xff);
	vstore4(pix, idx, image);
}
