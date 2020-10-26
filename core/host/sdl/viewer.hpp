#pragma once

#include <cstdint>
#include <functional>

#include <SDL2/SDL.h>

#include "base.hpp"


class Viewer {
    private:
    sdl::InitOnce sdl;

    int width;
    int height;
    SDL_Window *window;
    SDL_Renderer *renderer;
    SDL_Texture *texture;

    public:
    Viewer(int width=800, int height=600);
    ~Viewer();

    void display(std::function<void(uint8_t *data)> store);
};
