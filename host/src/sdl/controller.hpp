#pragma once

#include <unordered_map>
#include <functional>

#include "base.hpp"

#include <algebra/moebius.hpp>
#include <view.hpp>


class ControllerBase {
protected:
    std::unordered_map<sdl::Key, bool> keys;
    int mouse_x = 0, mouse_y = 0;
    int fov = 0, dof = 0;
    bool grab = false;
    bool ctrl = false, shift = false;

    real move_speed = 1.0; // 1/s
    real rot_speed = 2.0; // rad/s
    real mouse_sens = 2e-3; // rad/pix
    real wheel_sens = 0.1;

    int height;

public:
    bool handle();
    
    virtual bool step(real dt) = 0;

    void grab_mouse(bool g);
};


template <typename G>
class Controller : public ControllerBase {
private:
    static const std::unordered_map<
        sdl::Key,
        std::function<typename G::Map(real)>
    > MOVE_KEYS;
    static const std::unordered_map<
        sdl::Key,
        std::function<typename G::Map(real)>
    > ROT_KEYS;

public:
    View<G> view, view_prev;

    Controller() {
        for (auto& p : MOVE_KEYS) {
            keys[p.first] = false;
        }
        for (auto& p : ROT_KEYS) {
            keys[p.first] = false;
        }
    }
    Controller(const View<G> &v) : Controller() {
        view = v;
        view_prev = view;
    }

    bool step(double dt) override {
        bool still = true;
        view_prev = view;
        typename G::Map pos = view.position;

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
            pos *= G::yrotate(mouse_sens*mouse_x);
            mouse_x = 0;
            pos *= G::xrotate(mouse_sens*mouse_y);
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
};

template <typename G>
const std::unordered_map<
    sdl::Key,
    std::function<typename G::Map(real)>
> Controller<G>::MOVE_KEYS = {
    std::make_pair(SDLK_w, [](double l) { return G::zshift(l); }),
    std::make_pair(SDLK_a, [](double l) { return G::xshift(-l); }),
    std::make_pair(SDLK_s, [](double l) { return G::zshift(-l); }),
    std::make_pair(SDLK_d, [](double l) { return G::xshift(l); }),
    std::make_pair(SDLK_SPACE, [](double l) { return G::yshift(-l); }),
    std::make_pair(SDLK_c, [](double l) { return G::yshift(l); })
};
template <typename G>
const std::unordered_map<
    sdl::Key,
    std::function<typename G::Map(real)>
> Controller<G>::ROT_KEYS = {
    std::make_pair(SDLK_q, [](double a) { return G::zrotate(-a); }),
    std::make_pair(SDLK_e, [](double a) { return G::zrotate(a); })
};
