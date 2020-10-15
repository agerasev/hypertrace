#include "renderer.hpp"

#include <cstdint>
#include <vector>
#include <chrono>
#include <random>
#include <algorithm>

using namespace rstd;

using duration = std::chrono::duration<double>;


includer BasicRenderer::make_includer(const Config &) {
    includer inc(
        "device/render/euclidean.cl",
        std::list<std::string>{"src"},
        {},
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

BasicRenderer::BasicRenderer(
    cl_device_id device,
    int width, int height,
    const Config &config
) :
    width(width),
    height(height)
{
    context = Rc(cl::Context::create(device).expect("Context create error"));
    queue = cl::Queue::create(context, device).expect("Queue create error");

    auto prog_and_log = cl::Program::create(
        context, device,
        make_includer(config)
    );
    println_("Render build log: {}", prog_and_log.get<1>());
    program = Rc(prog_and_log.get<0>().expect("Program create error"));
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

void BasicRenderer::load_image(uint8_t *data) {
    image.load(queue, data).expect("Image load error");
}

int BasicRenderer::render_n(int n) {
    for (int i = 0; i < n; ++i) {
        render();
    };
    return n;
}

int BasicRenderer::render_for(double sec) {
    const duration render_time(sec);
    
    int sample_counter = 0;
    auto start = std::chrono::system_clock::now();
    do {
        render();
        sample_counter += 1;
    } while(std::chrono::system_clock::now() - start < render_time);

    return sample_counter;
}
