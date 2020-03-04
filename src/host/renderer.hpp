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
    
    cl::Buffer objects;
    cl::Buffer objects_prev;
    cl::Buffer objects_mask;
    int object_count = 0;

    static void store_objs_to_buf(
        cl::Queue &queue, cl::Buffer &buf,
        const std::vector<Object> &objs
    );

    int monte_carlo_counter = 0;

    ViewPk view, view_prev;

    public:
    Renderer(cl_device_id device, int width=800, int height=600);

    void store_objects(const std::vector<Object> &objs);
    void store_objects(
        const std::vector<Object> &objs,
        const std::vector<Object> &objs_prev,
        const std::vector<bool> &objs_mask
    );
    
    void load_image(uint8_t *data);

    void set_view(const View &v);
    void set_view(const View &v, const View &vp);

    void render(bool fresh);
    int render_n(int count, bool fresh);
    int render_for(double sec, bool fresh);
};
