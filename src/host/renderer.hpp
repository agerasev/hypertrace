#pragma once

#include <iostream>
#include <vector>
#include <cassert>
#include <cstdint>
#include <chrono>
#include <random>
#include <algorithm>

#include <opencl/opencl.hpp>

#include <traits.hpp>
#include <geometry.hpp>
#include <view.hpp>
//#include <object.hh>


// FIXME: Use `set_view()` method instead of `fresh` argument
template <typename Geo, typename Obj>
class Renderer {
public:
    static_assert(
        is_same<Geo, typename Obj::Geo>(),
        "Object and renderer geometry mismatch"
    );

    struct Config {
        struct Blur {
            bool lens = false;
            bool motion = false;
            bool object_motion = false;
        };

        int path_max_depth = 6;
        int path_max_diffuse_depth = 2;
        Blur blur;
        double gamma = 2.2;
    };

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
        const std::vector<Obj> &objs
    ) {
        std::vector<device_type<Obj>> pack(objs.size());
        for (size_t i = 0; i < objs.size(); ++i) {
            pack[i] = to_device(objs[i]);
        }
        buf.store(queue, pack.data(), sizeof(device_type<Obj>)*objs.size());
    }

    int monte_carlo_counter = 0;

    View<Geo> view, view_prev;
    
    using duration = std::chrono::duration<double>;

public:
    Renderer(
        cl_device_id device,
        const std::string &src,
        int width, int height
    ) :
        width(width),
        height(height),

        context(device),
        queue(context, device),

        program(context, device, src),
        kernel(program, "trace_kernel"),

        image(queue, width*height*4),
        screen(queue, width*height*3*sizeof(cl_float), true),

        seeds(queue, width*height*sizeof(cl_uint))
    {
        std::mt19937 rng(0xdeadbeef);
        std::vector<uint32_t> host_seeds(width*height);
        for (uint32_t &seed : host_seeds) {
            seed = rng();
        }
        seeds.store(queue, host_seeds.data());
    }
    
    void store_objects(const std::vector<Obj> &objs) {
        store_objects(
            objs,
            std::vector<Obj>(),
            std::vector<bool>(objs.size(), false)
        );
    }

    void store_objects(
        const std::vector<Obj> &objs,
        const std::vector<Obj> &objs_prev,
        const std::vector<bool> &objs_mask
    ) {
        store_objs_to_buf(queue, objects, objs);

        if (objs_prev.size() > 0) {
            assert(objs.size() == objs_prev.size());
            store_objs_to_buf(queue, objects_prev, objs_prev);
        }

        assert(objs.size() == objs_mask.size());
        std::vector<uchar> mask_pk(objs_mask.size());
        std::transform(
            objs_mask.begin(), objs_mask.end(),
            mask_pk.begin(), [](uchar x) { return uchar(x); }
        );
        objects_mask.store(queue, mask_pk.data(), mask_pk.size());

        object_count = objs.size();
    }
    
    void load_image(uint8_t *data) {
        image.load(queue, data);
    }

    void set_view(const View<Geo> &v) {
        set_view(v, v);
    }
    void set_view(const View<Geo> &v, const View<Geo> &vp) {
        view = v;
        view_prev = vp;
    }

    void render(bool fresh) {
        if (fresh) {
            monte_carlo_counter = 0;
        }

        kernel(
            queue, width*height,
            screen, image,
            width, height,
            monte_carlo_counter,
            seeds,

            to_device(view),
            //to_device(view_prev)

            objects,
            //objects_prev,
            //objects_mask, 
            object_count
        );

        monte_carlo_counter += 1;
    }

    int render_n(int n, bool fresh) {
        for (int i = 0; i < n; ++i) {
            render(fresh);
            fresh = false;
        };
        return n;
    }

    int render_for(double sec, bool fresh) {
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
};
