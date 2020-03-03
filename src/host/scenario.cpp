#include "scenario.hpp"

#include <memory>
#include <vector>
#include <cassert>
#include <algorithm>

#include <view.hh>

Point::Point(Moebius p, real fl) :
    position(p), focal_length(fl)
{}

Point Point::interpolate(
    const Point &a, const Point &b,
    double p
) {
    return Point(
        mo_chain(
            a.position,
            mo_pow(mo_chain(mo_inverse(a.position), b.position), p)
        ),
        a.focal_length*(1 - p) + b.focal_length*p
    );
}

Transition::Transition(double d) :
    duration(d)
{}

DelayTransition::DelayTransition(double d, Point p) :
    Transition(d), point(p)
{}
Point DelayTransition::get_point(double p) const {
    return point;
}

ConstantSpeedTransition::ConstantSpeedTransition(double d, Point a, Point b) :
    Transition(d), start(a), stop(b)
{}
Point ConstantSpeedTransition::get_point(double p) const {
    return Point::interpolate(start, stop, p);
}

SquareSpeedTransition::SquareSpeedTransition(double d, Point a, Point b, double at, double bt) :
    ConstantSpeedTransition(d, a, b), markers{at, bt}
{}
Point SquareSpeedTransition::get_point(double p) const {
    return Point::interpolate(
        start, stop,
        (3*(markers[0]*(1-p) + markers[1]*p)*(1-p) + p*p)*p
    );
}

void PathScenario::add_transition(std::unique_ptr<Transition> t) {
    index.push_back(std::make_pair(
        std::move(t),
        path_duration() + t->duration
    ));
}

PathScenario::PathScenario() = default;

View PathScenario::get_view(double t, double dt) const {
    Point p = get_point(t);
    Point dp = get_point(t - dt);

    View v;
    init_view(&v);
    v.position = p.position;
    v.motion = mo_chain(mo_inverse(p.position), dp.position);

    v.focal_length = p.focal_length;
    return v;
}

double PathScenario::path_duration() const {
    if (index.size() > 0) {
        return index.back().second;
    } else {
        return 0.0;
    }
}

double PathScenario::duration() const {
    return path_duration();
}

Point PathScenario::get_point(double time) const {
    assert(index.size() > 0);

    static const auto comp = [](
        const std::pair<std::unique_ptr<Transition>, double> &a,
        double b
    ) {
        return a.second < b;
    };

    auto i = std::lower_bound(index.begin(), index.end(), time, comp);
    if (i == index.end()) {
        return index.back().first->get_point(1.0);
    }

    const auto &t = i->first;
    double p = 1.0 - (i->second - time)/t->duration;
    if (p <= 0.0) {
        p = 0.0;
    }

    return t->get_point(p);
}
