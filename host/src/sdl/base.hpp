#pragma once

#include <atomic>
#include <exception>
#include <cstdint>
#include <cassert>

#include <SDL2/SDL.h>


namespace sdl {
    class InitOnce {
        private:
        static std::atomic_int COUNTER;

        public:
        InitOnce();
        ~InitOnce();

        InitOnce(const InitOnce &) = delete;
        InitOnce &operator=(const InitOnce &) = delete;
    };

    class TextureAccess {
        private:
        SDL_Texture *texture;
        uint8_t *_data = nullptr;
        int _pitch = 0;

        public:
        TextureAccess(SDL_Texture *texture);
        ~TextureAccess();
        uint8_t *data();
        int pitch();
    };

    typedef int Key;
}
