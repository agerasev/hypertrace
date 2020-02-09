#pragma once

#include <atomic>
#include <exception>
#include <cstdint>

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

namespace sdl {
    class InitOnce {
        private:
        static std::atomic_int COUNTER;

        public:
        InitOnce() {
            if (COUNTER.fetch_add(1) == 0) {
                assert(SDL_Init(SDL_INIT_VIDEO) >= 0);
            }
        }
        ~InitOnce() {
            if (COUNTER.fetch_sub(1) == 1) {
                SDL_Quit();
            }
        }

        InitOnce(const InitOnce &) = delete;
        InitOnce &operator=(const InitOnce &) = delete;
    };
    std::atomic_int InitOnce::COUNTER(0);

    class TextureAccess {
        private:
        SDL_Texture *texture;
        uint8_t *_data = nullptr;
        int _pitch = 0;

        public:
        TextureAccess(SDL_Texture *texture) : texture(texture) {
            assert(SDL_LockTexture(
                texture, nullptr,
                (void**)&_data, &_pitch
            ) == 0);
        }
        ~TextureAccess() {
            SDL_UnlockTexture(texture);
        }
        uint8_t *data() {
            return _data;
        }
        int pitch() {
            return _pitch;
        }
    };
}

class Viewer {
    private:
    sdl::InitOnce sdl;

    int width;
    int height;
    SDL_Window *window;
    SDL_Renderer *renderer;
    SDL_Texture *texture;

    public:
    Viewer(int width=800, int height=600) {
        this->width = width;
        this->height = height;

        window = SDL_CreateWindow(
            "HyperTrace",
            SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
            800, 600,
            SDL_WINDOW_RESIZABLE
        );
        assert(window != nullptr);

        renderer = SDL_CreateRenderer(window, -1, 0);
        assert(renderer != nullptr);
        
        texture = SDL_CreateTexture(
            renderer,
            SDL_PIXELFORMAT_RGBA32,
            SDL_TEXTUREACCESS_STREAMING,
            width, height
        );
        assert(texture != nullptr);
    }
    ~Viewer() {
        SDL_DestroyTexture(texture);
        SDL_DestroyRenderer(renderer);
        SDL_DestroyWindow(window);
    }

    bool display(std::function<void(uint8_t *data)> store) {
        SDL_Event event;

        while(SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                return false;
            }
        }

        {
            sdl::TextureAccess lock(texture);
            assert(lock.pitch() == 4*width);
            store(lock.data());
        }

        assert(SDL_RenderCopy(renderer, texture, nullptr, nullptr) == 0);

        SDL_RenderPresent(renderer);

        SDL_Delay(16);

        return true;
    }
};
