#include "base.hpp"


sdl::InitOnce::InitOnce() {
    if (COUNTER.fetch_add(1) == 0) {
        assert(SDL_Init(SDL_INIT_VIDEO) >= 0);
    }
}
sdl::InitOnce::~InitOnce() {
    if (COUNTER.fetch_sub(1) == 1) {
        SDL_Quit();
    }
}

std::atomic_int sdl::InitOnce::COUNTER(0);


sdl::TextureAccess::TextureAccess(SDL_Texture *texture) : texture(texture) {
    assert(SDL_LockTexture(
        texture, nullptr,
        (void**)&_data, &_pitch
    ) == 0);
}
sdl::TextureAccess::~TextureAccess() {
    SDL_UnlockTexture(texture);
}
uint8_t *sdl::TextureAccess::data() {
    return _data;
}
int sdl::TextureAccess::pitch() {
    return _pitch;
}
