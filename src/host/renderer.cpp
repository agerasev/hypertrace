#include "renderer.hpp"

#include <iostream>
#include <sstream>
#include <vector>
#include <cassert>
#include <cstdint>
#include <chrono>
#include <random>
#include <algorithm>

#define OCLGEN

using duration = std::chrono::duration<double>;

Renderer::Renderer(
    cl_device_id device,
    int width, int height,
    const Config &config
) :
    width(width),
    height(height),

    context(device),
    queue(context, device),

    program(
        context, device,
#ifndef OCLGEN
        "source.cl",
        {"src/device", "src/common"}
#else // OCLGEN
        "device.gen.cl",
        {"build/device"}
#endif // OCLGEN
    ),
    kernel(program, "render"),

    image(context, width*height*4),
    screen(context, width*height*3*sizeof(cl_float)),

    seeds(context, width*height*sizeof(cl_uint))
{
    std::mt19937 rng(0xdeadbeef);
    std::vector<uint32_t> host_seeds(width*height);
    for (uint32_t &seed : host_seeds) {
        seed = rng();
    }
    seeds.store(queue, host_seeds.data());

    set_view(view_init());
}

void Renderer::store_objs_to_buf(
    cl::Queue &queue, cl::Buffer &buf,
    const std::vector<Object> &objs
) {
    std::vector<ObjectPk> pack(objs.size());

    for (size_t i = 0; i < objs.size(); ++i) {
        pack_object(&pack[i], &objs[i]);
    }
    buf.store(queue, pack.data(), sizeof(ObjectPk)*objs.size());
}

void Renderer::store_objects(const std::vector<Object> &objs) {
    store_objects(
        objs,
        std::vector<Object>(),
        std::vector<bool>(objs.size(), false)
    );
}

void Renderer::store_objects(
    const std::vector<Object> &objs,
    const std::vector<Object> &objs_prev,
    const std::vector<bool> &objs_mask
) {
    store_objs_to_buf(queue, objects, objs);

    if (objs_prev.size() > 0) {
        assert(objs.size() == objs_prev.size());
        store_objs_to_buf(queue, objects_prev, objs_prev);
    }

    assert(objs.size() == objs_mask.size());
    std::vector<uchar_pk> mask_pk(objs_mask.size());
    std::transform(
        objs_mask.begin(), objs_mask.end(),
        mask_pk.begin(), [](bool x) { return (uchar_pk)x; }
    );
    objects_mask.store(queue, mask_pk.data(), mask_pk.size());

    object_count = objs.size();
}

void Renderer::load_image(uint8_t *data) {
    image.load(queue, data);
}

void Renderer::set_view(const View &v) {
    set_view(v, v);
}
void Renderer::set_view(const View &v, const View &vp) {
    view = view_pack(v);
    view_prev = view_pack(vp);
}

void Renderer::render(bool fresh) {
    if (fresh) {
        monte_carlo_counter = 0;
    }

    kernel(
        queue, width*height,
        screen, image,
        width, height,
        monte_carlo_counter,
        seeds,

        view, view_prev,

        objects, objects_prev,
        objects_mask, object_count
    );

    monte_carlo_counter += 1;
}

int Renderer::render_n(int n, bool fresh) {
    for (int i = 0; i < n; ++i) {
        render(fresh);
        fresh = false;
    };
    return n;
}

int Renderer::render_for(double sec, bool fresh) {
    const duration render_time(sec);
    
    int sample_counter = 0;
    auto start = std::chrono::system_clock::now();
    do {
        render(fresh);
        fresh = false;
        sample_counter += 1;
    } while(std::chrono::system_clock::now() - start < render_time);

    return sample_counter;
}
