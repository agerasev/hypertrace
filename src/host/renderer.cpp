#include "renderer.hpp"

#include <iostream>
#include <vector>
#include <cassert>
#include <cstdint>
#include <chrono>
#include <random>


using duration = std::chrono::duration<double>;

Renderer::Renderer(cl_device_id device, int width, int height) :
    width(width),
    height(height),

    context(device),
    queue(context, device),

    program(context, device, "render.cl", {"src/device", "src/common"}),
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

void Renderer::store_objects(const std::vector<Object> &objects) {
    std::vector<ObjectPk> package(objects.size());

    for (size_t i = 0; i < objects.size(); ++i) {
        pack_object(&package[i], &objects[i]);
    }
    object_buffer.store(queue, package.data(), sizeof(ObjectPk)*objects.size());
    
    object_count = objects.size();
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
        object_buffer, object_count
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
