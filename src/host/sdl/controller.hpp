#pragma once

#include <unordered_map>
#include <functional>

#include "base.hpp"

#include <view.hh>
#include <algebra/moebius.hh>


class Controller {
    private:
    static const std::unordered_map<sdl::Key, std::function<Moebius(double)>> MOVE_KEYS;
    static const std::unordered_map<sdl::Key, std::function<Moebius(double)>> ROT_KEYS;

    std::unordered_map<sdl::Key, bool> keys;
    int mouse_x = 0.0, mouse_y = 0.0;
    bool grab = false, update = false;
    bool ctrl = false, shift = false;

    double move_speed = 1.0; // 1/s
    double rot_speed = 2.0; // rad/s
    double mouse_sens = 2e-3; // rad/pix
    double wheel_sens = 0.1;

    int height;

    public:
    View view;

    Controller();
    Controller(const View &v);

    bool handle();
    bool step(double dt);

    void grab_mouse(bool g);
};
