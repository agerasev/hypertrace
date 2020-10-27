#pragma once

#include <memory>
#include <vector>
#include <cassert>
#include <algorithm>

#include <view.hpp>


template <typename G>
class Transition {
public:
    double duration = 1.0; // seconds

    Transition(double d) : duration(d) {}
    virtual ~Transition() = default;
    virtual View<G> get_view(double p) const = 0;
};

template <typename G>
class DelayTransition : public Transition<G> {
public:
    View<G> view;

    DelayTransition(double d, View<G> p) : Transition<G>(d), view(p) {}
    View<G> get_view(double p) const override {
        return view;
    }
};

template <typename G>
class LinearTransition : public Transition<G> {
public:
    View<G> start, stop;

    LinearTransition(double d, View<G> a, View<G> b) :
        Transition<G>(d), start(a), stop(b) {}
    View<G> get_view(double p) const override {
        return interpolate(start, stop, p);
    }
};

template <typename G>
class SquareTransition : public LinearTransition<G> {
public:
    double markers[2];

    SquareTransition(double d, View<G> a, View<G> b, double at, double bt) :
        LinearTransition<G>(d, a, b), markers{at, bt} {}
    View<G> get_view(double p) const override {
        return interpolate(
            this->start, this->stop,
            (3*(markers[0]*(1-p) + markers[1]*p)*(1-p) + p*p)*p
        );
    }
};

template <typename Obj>
class Scenario {
public:
    typedef typename Obj::Geo Geo;

    virtual double duration() const = 0;
    virtual View<Geo> get_view(double time) const = 0;
    virtual std::vector<Obj> get_objects(double time) const = 0;
};

template <typename Obj>
class PathScenario : public Scenario<Obj> {
public:
    typedef typename Scenario<Obj>::Geo Geo;

private:
    std::vector<std::pair<std::unique_ptr<Transition<Geo>>, double>> index;

public:
    void add_transition(std::unique_ptr<Transition<Geo>> t) {
        index.push_back(std::make_pair(
            std::move(t),
            path_duration() + t->duration
        ));
    }
    template <typename ... Args>
    void add_transition(std::unique_ptr<Transition<Geo>> t, Args ...args) {
        add_transition(std::move(t));
        add_transition(args...);
    }

public:
    PathScenario() = default;
    template <typename ... Args>
    PathScenario(Args ...args) : PathScenario() {
        add_transition(args...);
    }

    double path_duration() const {
        if (index.size() > 0) {
            return index.back().second;
        } else {
            return 0.0;
        }
    }

    double duration() const override {
        return path_duration();
    }
    View<Geo> get_view(double time) const override {
        assert(index.size() > 0);

        static const auto comp = [](
            const std::pair<std::unique_ptr<Transition<Geo>>, double> &a,
            double b
        ) {
            return a.second < b;
        };

        auto i = std::lower_bound(index.begin(), index.end(), time, comp);
        if (i == index.end()) {
            return index.back().first->get_view(1.0);
        }

        const auto &t = i->first;
        double p = 1.0 - (i->second - time)/t->duration;
        if (p <= 0.0) {
            p = 0.0;
        }

        return t->get_view(p);
    }
};
