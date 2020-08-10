#include "controller.hpp"


bool ControllerBase::handle() {
    SDL_Event e;

    while(SDL_PollEvent(&e)) {
        if (
            e.type == SDL_QUIT ||
            (e.type == SDL_KEYDOWN && e.key.keysym.sym == SDLK_ESCAPE)
        ) {
            return false;
        }
        
        if (e.type == SDL_KEYDOWN || e.type == SDL_KEYUP) {
            bool down = (e.type == SDL_KEYDOWN);
            auto it = keys.find(e.key.keysym.sym);
            if (it != keys.end()) {
                it->second = down;
            }
            switch (e.key.keysym.sym) {
                case SDLK_LSHIFT:
                    shift = down;
                    break;
                case SDLK_LCTRL:
                    ctrl = down;
                    break;
            }
        } else if (e.type == SDL_MOUSEMOTION && grab) {
            mouse_x += e.motion.xrel;
            mouse_y -= e.motion.yrel;
        }

        if (e.type == SDL_KEYDOWN) {
            switch (e.key.keysym.sym) {
                case SDLK_TAB:
                    grab_mouse(!grab);
                    break;
            }
        }
        if (e.type == SDL_MOUSEWHEEL) {
            if (e.wheel.y) {
                if (ctrl) {
                    dof += e.wheel.y;
                } else {
                    fov += e.wheel.y;
                }
            }
        }
    }

    return true;
}

void ControllerBase::grab_mouse(bool g) {
    grab = g;
    SDL_SetRelativeMouseMode((SDL_bool)g);
}
