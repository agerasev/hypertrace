#pragma once

#include "base.hpp"

#include <SDL2/SDL_image.h>


namespace sdl {
    void save_image(
        std::string filename,
        int width, int height,
        std::function<void(uint8_t *data)> store
    ) {
        // FIXME: RAII Surface wrapper
        SDL_Surface *surface = SDL_CreateRGBSurface(
            0, width, height, 32,
            0xff<<0, 0xff<<8, 0xff<<16, 0xff<<24
        );
        assert(surface != nullptr);
        assert(surface->pitch == 4*width);

        store((uint8_t*)surface->pixels);

        assert(IMG_SavePNG(surface, filename.c_str()) == 0);

        SDL_FreeSurface(surface);
    }
};
