#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cstring>
#include <cmath>

#include <SDL2/SDL.h>

double clip(double x, double a=0.0, double b=1.0) {
    if (x < a) {
        return a;
    } else if (x > b) {
        return b;
    } else {
        return x;
    }
}

int main(int argc, const char *argv[]) {
    SDL_Window *window = nullptr;
    SDL_Renderer *renderer = nullptr;
    SDL_Texture *texture = nullptr;

    assert(SDL_Init(SDL_INIT_VIDEO) >= 0);

    window = SDL_CreateWindow(
        "HyperTrace",
        SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
        800, 600,
        SDL_WINDOW_RESIZABLE
    );
    assert(window != nullptr);

    int width = 800, height = 600;
    renderer = SDL_CreateRenderer(window, -1, 0);
    assert(renderer != nullptr);
    texture = SDL_CreateTexture(
        renderer,
        SDL_PIXELFORMAT_RGBA8888,
        SDL_TEXTUREACCESS_STREAMING,
        width, height
    );
    assert(texture != nullptr);


    const double freqs[2][3] = {{3.0, 4.0, 5.0}, {6.0, 7.0, 8.0}};
    SDL_Event event;
    double t = 0.0;

    while(1) {
        bool done = false;
        while(SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                done = true;
            }
        }
        if (done) {
            break;
        }

        uint8_t *data = nullptr;
        int pitch = 0;
        SDL_LockTexture(texture, nullptr, (void**)&data, &pitch);

        assert(pitch == 4*width);
        for (int iy = 0; iy < height; ++iy) {
            for (int ix = 0; ix < width; ++ix) {
                uint8_t *pix = data + (iy*width + ix)*4;
                pix[0] = 0xff;
                for (int j = 0; j < 3; ++j) {
                    double x = double(ix)/(width - 1);
                    double y = double(iy)/(height - 1);
                    pix[j + 1] = 255*clip(x*sin(freqs[0][j]*t) + y*sin(freqs[1][j]*t));
                }
            }
        }

        SDL_UnlockTexture(texture);

        SDL_RenderCopy(renderer, texture, nullptr, nullptr);

        SDL_RenderPresent(renderer);

        SDL_Delay(40);
        t += 1e-2;
    }

    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}

/*
#include "cl.hpp"

int main(int argc, const char *argv[]) {
	std::vector<cl_platform_id> platforms = cl::get_platforms();
    std::vector<std::vector<cl_device_id>> platform_devices(platforms.size());

    for (size_t i = 0; i < platforms.size(); ++i) {
        std::cout << std::endl << "Platform " << i << ":" << std::endl;
        cl::print_platform_info(platforms[i], "  ");

        std::vector<cl_device_id> devices = cl::get_devices(platforms[i]);
        for (size_t j = 0; j < devices.size(); ++j) {
            std::cout << std::endl << "  Device " << j << ":" << std::endl;
            cl::print_device_info(devices[j], "    ");
        }
        platform_devices[i] = std::move(devices);
    }
    std::cout << std::endl;

    int platform_no = 0;
    int device_no = 0;

    try {
        if (argc >= 2) {
            platform_no = std::stoi(argv[1]);
            if (argc >= 3) {
                device_no = std::stoi(argv[2]);
            }
        }
    } catch(...) {
        std::cerr << "Invalid argument" << std::endl;
        exit(1);
    }

    std::cout << "Using platform " << platform_no << ", device " << device_no << std::endl;
    
    assert(platform_no >= 0 && platform_no < int(platforms.size()));
    cl_platform_id platform = platforms[0];
    assert(device_no >= 0 && device_no < int(platform_devices[platform_no].size()));
    cl_device_id device = platform_devices[platform_no][device_no];

	cl::Context context(device);
	cl::Queue queue(context, device);

    cl::Program program(context, device, "device/kernel.cl", "src");
    cl::Kernel kernel(program, "_increment");

    const char in_data[7] = "ABCdef";
    char out_data[7] = {'\0'};
    size_t size = 6;

    cl::Buffer in_buffer(context, size);
    cl::Buffer out_buffer(context, size);

    in_buffer.store(queue, in_data);

    kernel(queue, size, in_buffer.raw(), out_buffer.raw());

    out_buffer.load(queue, out_data);

    std::cout << in_data << std::endl << out_data << std::endl;
    for (int i = 0; i < 6; ++i) {
        assert(in_data[i] + 1 == out_data[i]);
    }

    return 0;
}
*/
