#pragma once

#include <memory>
#include <vector>
#include <cassert>
#include <algorithm>

#include <view.hh>
#include <object.hh>


class Transition {
    public:
    double duration = 1.0; // seconds

    Transition(double d);
    virtual View get_view(double p) const = 0;
};

class DelayTransition : public Transition {
    public:
    View view;

    DelayTransition(double d, View p);
    View get_view(double p) const override;
};

class LinearTransition : public Transition {
    public:
    View start, stop;

    LinearTransition(double d, View a, View b);
    View get_view(double p) const override;
};

class SquareTransition : public LinearTransition {
    public:
    double markers[2];

    SquareTransition(double d, View a, View b, double at, double bt);
    View get_view(double p) const override;
};

class Scenario {
    public:
    virtual double duration() const = 0;
    virtual View get_view(double time) const = 0;
    virtual std::vector<Object> get_objects(double time) const = 0;
};

class PathScenario : public Scenario {
    private:
    std::vector<std::pair<std::unique_ptr<Transition>, double>> index;

    public:
    void add_transition(std::unique_ptr<Transition> t);
    template <typename ... Args>
    void add_transition(std::unique_ptr<Transition> t, Args ...args) {
        add_transition(t);
        add_transition(args...);
    }

    public:
    PathScenario();
    template <typename ... Args>
    PathScenario(Args ...args) : PathScenario() {
        add_transition(args...);
    }

    double path_duration() const;

    double duration() const override;
    View get_view(double time) const override;
};
