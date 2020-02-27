#pragma once

#include <view.hh>


class Scenario {
    public:
    virtual void step(double dt) = 0;
};
