#pragma once

#include <algebra/vector.hpp>
#include <random.hpp>
#include "path.hpp"


class Context {
public:
    Rng &rng;
    PathInfo path;
};
