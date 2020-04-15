#include "controller.hpp"

#include <geometry/hyperbolic.hpp>


const std::unordered_map<sdl::Key, std::function<Moebius(double)>> Controller::MOVE_KEYS = {
    std::make_pair(SDLK_w, [](double l) { return Hy::zshift(l); }),
    std::make_pair(SDLK_a, [](double l) { return Hy::xshift(-l); }),
    std::make_pair(SDLK_s, [](double l) { return Hy::zshift(-l); }),
    std::make_pair(SDLK_d, [](double l) { return Hy::xshift(l); }),
    std::make_pair(SDLK_SPACE, [](double l) { return Hy::yshift(-l); }),
    std::make_pair(SDLK_c, [](double l) { return Hy::yshift(l); })
};
const std::unordered_map<sdl::Key, std::function<Moebius(double)>> Controller::ROT_KEYS = {
    std::make_pair(SDLK_q, [](double a) { return Hy::zrotate(-a); }),
    std::make_pair(SDLK_e, [](double a) { return Hy::zrotate(a); })
};


Controller::Controller() {
    for (auto& p : MOVE_KEYS) {
        keys[p.first] = false;
    }
    for (auto& p : ROT_KEYS) {
        keys[p.first] = false;
    }
}

Controller::Controller(const View &v) : Controller() {
    view = v;
    view_prev = v;
}


bool Controller::handle() {
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

bool Controller::step(double dt) {
    bool still = true;
    view_prev = view;
    Moebius pos = view.position;

    for (auto& p : MOVE_KEYS) {
        if (keys[p.first]) {
            pos *= p.second(move_speed*dt);
            still = false;
        }
    }
    for (auto& p : ROT_KEYS) {
        if (keys[p.first]) {
            pos *= p.second(rot_speed*dt);
            still = false;
        }
    }

    if (mouse_x != 0 || mouse_y != 0) {
        pos *= Hy::xrotate(mouse_sens*mouse_x);
        mouse_x = 0;
        pos *= Hy::yrotate(mouse_sens*mouse_y);
        mouse_y = 0;
        still = false;
    }

    view.position = pos;

    if (fov || dof) {
        view.focal_length *= math::exp(wheel_sens*fov);
        view.field_of_view *= math::exp(wheel_sens*dof);
        fov = 0;
        dof = 0;
        still = false;
    }
    return !still;
}

void Controller::grab_mouse(bool g) {
    grab = g;
    SDL_SetRelativeMouseMode((SDL_bool)g);
}
