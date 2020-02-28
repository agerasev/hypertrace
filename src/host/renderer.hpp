#pragma once

#include <vector>
#include <cstdint>

#include <opencl/opencl.hpp>

#include <view.hh>
#include <object.hh>

// FIXME: Add `set_view()` method and use it instead of `fresh` argument
class Renderer {
    private:
    int width, height;

    cl::Context context;
    cl::Queue queue;

    cl::Program program;
    cl::Kernel kernel;

    cl::Buffer image;
    cl::Buffer screen;

    cl::Buffer seeds;
    
    cl::Buffer object_buffer;
    int object_count = 0;

    int monte_carlo_counter = 0;

    public:
    Renderer(cl_device_id device, int width=800, int height=600);

    void store_objects(const std::vector<Object> &objects);
    void load_image(uint8_t *data);

    void render(const View &view, bool fresh);
    int render_for(const View &view, double sec, bool fresh);
};
