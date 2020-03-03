#pragma once

#include <memory>
#include <vector>
#include <cassert>
#include <algorithm>

#include <view.hh>
#include <object.hh>


class Point {
    public:
    Moebius position;
    real focal_length;

    Point(Moebius p = mo_identity(), real fl = 4.0);
    static Point interpolate(const Point &a, const Point &b, double p);
};

class Transition {
    public:
    double duration = 1.0; // seconds

    Transition(double d);
    virtual Point get_point(double p) const = 0;
};

class DelayTransition : public Transition {
    public:
    Point point;

    DelayTransition(double d, Point p);
    Point get_point(double p) const override;
};

class ConstantSpeedTransition : public Transition {
    public:
    Point start, stop;

    ConstantSpeedTransition(double d, Point a, Point b);
    Point get_point(double p) const override;
};

class SquareSpeedTransition : public ConstantSpeedTransition {
    public:
    double markers[2];

    SquareSpeedTransition(double d, Point a, Point b, double at, double bt);
    Point get_point(double p) const override;
};

class Scenario {
    public:
    virtual double duration() const = 0;
    virtual View get_view(double t, double dt) const = 0;
    virtual const std::vector<Object> &get_objects(double t) const = 0;
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

    Point get_point(double time) const;
    double path_duration() const;

    double duration() const override;
    View get_view(double t, double dt) const override;
};
