#pragma once

#include <atomic>
#include <exception>
#include <cstdint>
#include <unordered_map>
#include <functional>

#include <SDL2/SDL.h>

#include <moebius.h>


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

    typedef int Key;
}

class Controller {
    private:
    static const std::unordered_map<sdl::Key, std::function<Moebius(double)>> MOVE_KEYS;
    static const std::unordered_map<sdl::Key, std::function<Moebius(double)>> ROT_KEYS;

    int height;
    std::unordered_map<sdl::Key, bool> keys;
    double mouse_x = 0.0, mouse_y = 0.0;
    Moebius map_;

    public:
    double move_speed = 1.0; // 1/s
    double rot_speed = 2.0; // rad/s
    double mouse_sens = 2.0; // rad/window_height

    Controller(int h) : Controller(h, Moebius::identity()) {}
    Controller(int h, const Moebius &m) : height(h), map_(m) {
        for (auto& p : MOVE_KEYS) {
            keys[p.first] = false;
        }
        for (auto& p : ROT_KEYS) {
            keys[p.first] = false;
        }
    }
    void handle(const SDL_Event &e) {
        if (e.type == SDL_KEYDOWN || e.type == SDL_KEYUP) {
            auto it = keys.find(e.key.keysym.sym);
            if (it != keys.end()) {
                it->second = (e.type == SDL_KEYDOWN);
            }
        } else if (e.type == SDL_MOUSEMOTION) {
            mouse_x += (double)e.motion.xrel/height;
            mouse_y -= (double)e.motion.yrel/height;
        }
    }
    void step(double dt) {
        for (auto& p : MOVE_KEYS) {
            if (keys[p.first]) {
                map_ *= p.second(move_speed*dt);
            }
        }
        for (auto& p : ROT_KEYS) {
            if (keys[p.first]) {
                map_ *= p.second(rot_speed*dt);
            }
        }
        map_ *= Moebius::xcoil(mouse_sens*mouse_x);
        mouse_x = 0.0;
        map_ *= Moebius::ycoil(mouse_sens*mouse_y);
        mouse_y = 0.0;
    }
    const Moebius &map() const {
        return map_;
    }
};

const std::unordered_map<sdl::Key, std::function<Moebius(double)>> Controller::MOVE_KEYS = {
    std::make_pair(SDLK_w, [](double l) { return Moebius::zshift(l); }),
    std::make_pair(SDLK_a, [](double l) { return Moebius::xshift(-l); }),
    std::make_pair(SDLK_s, [](double l) { return Moebius::zshift(-l); }),
    std::make_pair(SDLK_d, [](double l) { return Moebius::xshift(l); }),
    std::make_pair(SDLK_SPACE, [](double l) { return Moebius::yshift(-l); }),
    std::make_pair(SDLK_LSHIFT, [](double l) { return Moebius::yshift(l); })
};
const std::unordered_map<sdl::Key, std::function<Moebius(double)>> Controller::ROT_KEYS = {
    std::make_pair(SDLK_q, [](double a) { return Moebius::zrotate(-a); }),
    std::make_pair(SDLK_e, [](double a) { return Moebius::zrotate(a); })
};

class Viewer {
    private:
    sdl::InitOnce sdl;

    int width;
    int height;
    SDL_Window *window;
    SDL_Renderer *renderer;
    SDL_Texture *texture;

    Controller ctrl;
    int delay_ms = 16;

    public:
    Viewer(int width=800, int height=600) : ctrl(height) {
        this->width = width;
        this->height = height;

        window = SDL_CreateWindow(
            "HyperTrace",
            SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
            width, height,
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

        SDL_SetRelativeMouseMode((SDL_bool)true);
    }
    ~Viewer() {
        SDL_DestroyTexture(texture);
        SDL_DestroyRenderer(renderer);
        SDL_DestroyWindow(window);
    }

    bool display(std::function<void(uint8_t *data)> store) {
        SDL_Event event;

        while(SDL_PollEvent(&event)) {
            if (
                event.type == SDL_QUIT ||
                (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE)
            ) {
                return false;
            }
            ctrl.handle(event);
        }

        {
            sdl::TextureAccess lock(texture);
            assert(lock.pitch() == 4*width);
            store(lock.data());
        }

        assert(SDL_RenderCopy(renderer, texture, nullptr, nullptr) == 0);

        SDL_RenderPresent(renderer);

        SDL_Delay(delay_ms);

        ctrl.step(1e-3*delay_ms);

        return true;
    }

    const Controller &controller() const {
        return ctrl;
    }
};
