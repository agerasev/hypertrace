#pragma once

#include <host/opencl/opencl.hpp>

#include <common/types.hh>
#include <common/render/view.hh>
//#include <object.hh>


struct Config {};

class BasicRenderer {
private:
    includer make_includer(
        const Config &cfg,
        const std::string &src
    );

protected:
    int width, height;

    rstd::Rc<cl::Context> context;
    cl::Queue queue;

    rstd::Rc<cl::Program> program;
    cl::Kernel kernel;

    cl::Buffer image;
    cl::Buffer screen;

    //cl::Buffer object;

    cl::Buffer seeds;
    int monte_carlo_counter = 0;
    bool fresh = true;

public:
    BasicRenderer(
        cl_device_id device,
        int width, int height,
        const Config &config,
        const std::string &src
    );

    void load_image(uint8_t *data);

    virtual void render() = 0;
    int render_n(int n);
    int render_for(double sec);
};

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
class Renderer : public BasicRenderer {
public:
    View<G> view;

public:
    Renderer(
        cl_device_id device,
        int width, int height,
        const Config &config={}
    ) :
        BasicRenderer(
            device,
            width, height,
            config,
            RenderFile<G>::path()
        )
    {}
    
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


    void render() override {
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
};
