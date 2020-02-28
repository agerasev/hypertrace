#pragma once

#include <memory>
#include <vector>
#include <cassert>
#include <algorithm>

#include <view.hh>


class Scenario {
    public:
    class Point {
        public:
        Moebius position = mo_identity();
        real focal_length = 4.0;
    };
    class Transition {
        public:
        real duration = 1.0; // seconds
    };

    private:
    class PointNode;
    class TransitionNode;

    class PointNode {
        public:
        Point base;
        std::shared_ptr<TransitionNode> prev, next;
        PointNode(const Point &p) : base(p) {}
    };
    class TransitionNode {
        public:
        Transition base;
        std::shared_ptr<PointNode> prev, next;
        TransitionNode(const Transition &t) : base(t) {}
    };

    private:
    std::shared_ptr<PointNode> start, stop;
    std::vector<std::pair<std::shared_ptr<PointNode>, double>> index;

    void set_start_point(const Point &p);

    public:
    void add_transition(const Transition &t, const Point &p);
    template <typename ... Args>
    void add_transition(const Transition &t, const Point &p, const Args &...args) {
        add_transition(t, p);
        add_transition(args...);
    }

    public:
    Scenario(const Point &p);
    template <typename ... Args>
    Scenario(const Point &p, const Args &...args) : Scenario(p) {
        add_transition(args...);
    }

    Point get_point(double time) const;
    View get_view(double t, double dt) const;
    double duration() const;
};
