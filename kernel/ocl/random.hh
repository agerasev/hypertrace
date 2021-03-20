#pragma once

#include <types.hh>
#include <algebra/real.hh>
#include <algebra/vector.hh>


typedef struct Rng {
    uint state;
} Rng;

Rng rng_init(uint seed);
uint rng_next(Rng *rng);

real random_uniform(Rng *rng);
real2 random_uniform2(Rng *rng);
real3 random_uniform3(Rng *rng);
real4 random_uniform4(Rng *rng);

real3 random_sphere(Rng *rng);
real3 random_hemisphere(Rng *rng);
real3 random_hemisphere_cosine(Rng *rng);
real3 random_sphere_cap(Rng *rng, real cos_alpha);


#ifdef UNITTEST

#include <vector>
#include <cassert>

template <typename T>
class Grid {
private:
    std::vector<real> cells;

public:
    virtual std::vector<real> sizes() const = 0;
    virtual size_t index(T x) const = 0;

public:
    Grid(size_t N) : cells(N, 0.0) {}

    real &operator[](T x) {
        size_t i = index(x);
        assert(i < cells.size());
        return cells[i];
    }
    std::vector<real> total(real norm) const {
        std::vector<real> total(cells.size());
        std::vector<real> sizes = this->sizes();
        for (size_t i = 0; i < cells.size(); ++i) {
            total[i] = cells[i]/sizes[i]/norm;
        }
        return total;
    }
    [[nodiscard]]
    bool check_all(real norm, real eps) const {
        std::vector<real> total = this->total(norm);
        std::vector<real> sizes = this->sizes();
        for (size_t i = 0; i < total.size(); ++i) {
            if (total[i] != approx(1).epsilon(eps/sizes[i])) {
                return false;
            }
        }
        return true;
    }
};

class SphereGrid : public Grid<real3> {
private:
    size_t P, T;
    real max_theta;

public:
    virtual std::vector<real> sizes() const override;
    virtual size_t index(real3 x) const override;
public:
    inline SphereGrid(
        size_t P, size_t T,
        real mt=PI
    ) :
        Grid<real3>(P*T),
        P(P), T(T),
        max_theta(mt)
    {}
};

class DiskGrid : public Grid<real2> {
private:
    size_t P, R;

public:
    virtual std::vector<real> sizes() const override;
    virtual size_t index(real2 x) const override;

public:
    inline DiskGrid(
        size_t P, size_t R
    ) :
        Grid<real2>(P*R),
        P(P), R(R)
    {}
};

#endif // UNITTEST

#ifndef HOST
#include "random.cc"
#endif // HOST
