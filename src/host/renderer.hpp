#pragma once

#include <cstdint>
#include <vector>
#include <chrono>
#include <random>
#include <algorithm>

#include <common/types.hh>
#include <common/render/view.hh>
#include <common/object/shape.hpp>

#include <host/opencl/opencl.hpp>


struct Config {};

template <typename G>
struct RenderFile {};
template <>
struct RenderFile<Eu> {
    static std::string path() {
        return "device/render/euclidean.cl";
    }
};
template <>
struct RenderFile<Hy> {
    static std::string path() {
        return "device/render/hyperbolic.cl";
    }
};

template <typename G>
class Renderer {
private:
    includer make_includer(
        const Config &,
        const std::string &source,
        std::map<std::string, std::string> &&files={}
    ) {
        assert_(files.insert(std::make_pair("main.cl", source)).second);

        includer inc(
            "main.cl",
            std::list<std::string>{"src"},
            files,
            std::map<std::string, bool>{
                std::make_pair("HOST", false),
                std::make_pair("INTEROP", false),
                std::make_pair("TEST", false),
                std::make_pair("TEST_UNIT", false),
                std::make_pair("TEST_DEV", false)
            }
        );

        bool status = inc.include();
        println_(inc.log());
        assert_(status);

        return inc;
    }

    rstd::Rc<cl::Context> context;
    cl::Queue queue;

    int width, height;

    rstd::Rc<cl::Program> program;
    cl::Kernel kernel;

    cl::Buffer image;
    cl::Buffer screen;

    //cl::Buffer object;

    cl::Buffer seeds;
    int monte_carlo_counter = 0;
    bool fresh = true;

    View<G> view;
    rstd::Box<Shape<G>> type;

    using duration = std::chrono::duration<double>;

public:
    Renderer(
        cl_device_id device,
        rstd::Rc<cl::Context> context_,
        int width_, int height_,
        rstd::Box<Shape<G>> &&type_,
        const Config &config={}
    ) :
        context(context_),
        width(width_),
        height(height_),
        type(std::move(type_))
    {
        queue = cl::Queue::create(context, device).expect("Queue create error");

        dyn::Source src = type->source();
        std::string source = format_(
            "#include <{}>\n"
            "#define object_detect {}_detect\n"
            "#include <{}>\n"
            ,
            src.name(),
            type->prefix(),
            RenderFile<G>::path()
        );
        auto prog_and_log = cl::Program::create(
            context, device,
            make_includer(config, source, src.into_files())
        );
        println_("Render build log: {}", prog_and_log.template get<1>());
        program = rstd::Rc(prog_and_log.template get<0>().expect("Program create error"));
        kernel = cl::Kernel::create(program, "render").expect("Kernel create error");

        image = cl::Buffer::create(queue, width*height*4).expect("Image buffer create error");
        screen = cl::Buffer::create(queue, width*height*3*sizeof(cl_float), true).expect("Screen buffer create error");

        seeds = cl::Buffer::create(queue, width*height*sizeof(cl_uint)).expect("Seed buffer create error");
        std::mt19937 rng(0xdeadbeef);
        std::vector<uint32_t> host_seeds(width*height);
        for (uint32_t &seed : host_seeds) {
            seed = rng();
        }
        seeds.store(queue, host_seeds.data()).expect("Seed buffer store error");
    }
    
    /*
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
    */

    void set_view(const View<G> &v) {
        view = v;
        fresh = true;
    }

    void load_image(uint8_t *data) {
        image.load(queue, data).expect("Image load error");
    }

    void render() {
        if (fresh) {
            monte_carlo_counter = 0;
            fresh = false;
        }

        dev_type<View<G>> view_dev;
        dev_store(&view_dev, &view);
        kernel(
            queue, width*height,
            screen, image,
            width, height,
            monte_carlo_counter,
            seeds,
            view_dev
            //objects,
            //object_count
        ).expect("Kernel run error");

        monte_carlo_counter += 1;
    }

    int render_n(int n) {
        for (int i = 0; i < n; ++i) {
            render();
        };
        return n;
    }

    int render_for(double sec) {
        const duration render_time(sec);
        
        int sample_counter = 0;
        auto start = std::chrono::system_clock::now();
        do {
            render();
            sample_counter += 1;
        } while(std::chrono::system_clock::now() - start < render_time);

        return sample_counter;
    }
};
