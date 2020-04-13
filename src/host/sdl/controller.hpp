#pragma once

#include <unordered_map>
#include <functional>

#include "base.hpp"

#include <algebra/moebius.hpp>
#include <view.hpp>


class Controller {
    private:
    static const std::unordered_map<sdl::Key, std::function<Moebius(real)>> MOVE_KEYS;
    static const std::unordered_map<sdl::Key, std::function<Moebius(real)>> ROT_KEYS;

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
    View view, view_prev;

    Controller();
    Controller(const View &v);

    bool handle();
    bool step(real dt);

    void grab_mouse(bool g);
};
