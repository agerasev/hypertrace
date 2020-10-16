#pragma once

#include <rstd/prelude.hpp>
#include <common/geometry/geometry.hpp>
#include "type.hpp"


// Base class for all objects
template <typename G>
struct Object : public virtual Type {
    // Storage for state between detection and interaction.
    virtual Box<Type> cache() const = 0;

    // Allow repeated collisions or not
    virtual bool repeat_allowed() = 0;
};
