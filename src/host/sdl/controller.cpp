#include "controller.hpp"

#include <geometry/hyperbolic.hh>


const std::unordered_map<sdl::Key, std::function<Moebius(double)>> Controller::MOVE_KEYS = {
    std::make_pair(SDLK_w, [](double l) { return hy_zshift(l); }),
    std::make_pair(SDLK_a, [](double l) { return hy_xshift(-l); }),
    std::make_pair(SDLK_s, [](double l) { return hy_zshift(-l); }),
    std::make_pair(SDLK_d, [](double l) { return hy_xshift(l); }),
    std::make_pair(SDLK_SPACE, [](double l) { return hy_yshift(-l); }),
    std::make_pair(SDLK_LSHIFT, [](double l) { return hy_yshift(l); })
};
const std::unordered_map<sdl::Key, std::function<Moebius(double)>> Controller::ROT_KEYS = {
    std::make_pair(SDLK_q, [](double a) { return hy_zrotate(-a); }),
    std::make_pair(SDLK_e, [](double a) { return hy_zrotate(a); })
};


Controller::Controller(int h) : height(h) {
    for (auto& p : MOVE_KEYS) {
        keys[p.first] = false;
    }
    for (auto& p : ROT_KEYS) {
        keys[p.first] = false;
    }
    init_view(&view);
}

Controller::Controller(int h, const View &v) : Controller(h) {
    view = v;
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
            auto it = keys.find(e.key.keysym.sym);
            if (it != keys.end()) {
                it->second = (e.type == SDL_KEYDOWN);
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
                view.fov *= exp(wheel_sens*e.wheel.y);
                update = true;
            }
        }
    }

    return true;
}

bool Controller::step(double dt) {
    bool still = true;

    Moebius pos = view.position;
    Moebius prev_pos = pos;

    for (auto& p : MOVE_KEYS) {
        if (keys[p.first]) {
            pos = mo_chain(pos, p.second(move_speed*dt));
            still = false;
        }
    }
    for (auto& p : ROT_KEYS) {
        if (keys[p.first]) {
            pos = mo_chain(pos, p.second(rot_speed*dt));
            still = false;
        }
    }

    if (mouse_x != 0 || mouse_y != 0) {
        pos = mo_chain(pos, hy_xrotate(mouse_sens*mouse_x/height));
        mouse_x = 0;
        pos = mo_chain(pos, hy_yrotate(mouse_sens*mouse_y/height));
        mouse_y = 0;
        still = false;
    }
    
    view.position = pos;
    if (!still) {
        view.motion = mo_inverse(mo_chain(mo_inverse(prev_pos), pos));
    } else {
        view.motion = mo_identity();
    }

    if (update) {
        still = false;
        update = false;
    }

    return !still;
}

void Controller::grab_mouse(bool g) {
    grab = g;
    SDL_SetRelativeMouseMode((SDL_bool)g);
}

const View &Controller::get_view() const {
    return view;
}
