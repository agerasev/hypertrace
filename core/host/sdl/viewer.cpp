#include "viewer.hpp"


Viewer::Viewer(int width, int height) {
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
}
Viewer::~Viewer() {
    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
}

void Viewer::display(std::function<void(uint8_t *data)> store) {
    {
        sdl::TextureAccess lock(texture);
        assert(lock.pitch() == 4*width);
        store(lock.data());
    }

    assert(SDL_RenderCopy(renderer, texture, nullptr, nullptr) == 0);

    SDL_RenderPresent(renderer);
}
