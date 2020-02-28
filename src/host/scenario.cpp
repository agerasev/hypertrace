#include "scenario.hpp"

#include <memory>
#include <vector>
#include <cassert>
#include <algorithm>

#include <view.hh>


void Scenario::set_start_point(const Point &p) {
    assert(!start && !stop);
    start = std::make_shared<PointNode>(p);
    stop = start;
    index.push_back(std::make_pair(start, 0.0));
}

void Scenario::add_transition(const Transition &t, const Point &p) {
    assert(start && stop);

    auto tn = std::make_shared<TransitionNode>(t);
    auto pn = std::make_shared<PointNode>(p);

    stop->next = tn;
    tn->prev = stop;
    tn->next = pn;
    pn->prev = tn;

    index.push_back(std::make_pair(pn, index.back().second + tn->base.duration));
    stop = pn;
}


Scenario::Scenario(const Point &p) {
    set_start_point(p);
}

Scenario::Point Scenario::get_point(double time) const {
    static const auto comp = [](
        const std::pair<std::shared_ptr<PointNode>, double> &a,
        double b
    ) {
        return a.second < b;
    };
    auto i = std::lower_bound(index.begin(), index.end(), time, comp);
    if (i != index.begin()) {
        --i;
    }
    double tt = time - i->second;
    std::shared_ptr<PointNode> apn = i->first;
    if (tt < 0.0 || !apn->next) {
        return apn->base;
    }
    std::shared_ptr<TransitionNode> tn = apn->next;
    std::shared_ptr<PointNode> bpn = tn->next;

    const Point &a = apn->base;
    const Point &b = bpn->base;
    const Transition &t = tn->base;
    double p = tt/t.duration;

    return Point {
        .position = mo_chain(
            a.position,
            mo_pow(mo_chain(mo_inverse(a.position), b.position), p)
        ),
        .focal_length = a.focal_length + (b.focal_length - a.focal_length)*p
    };
}

View Scenario::get_view(double t, double dt) const {
    Point p = get_point(t);
    Point dp = get_point(t - dt);
    View v;
    init_view(&v);
    v.position = p.position;
    v.motion = mo_inverse(mo_chain(mo_inverse(dp.position), p.position));
    v.focal_length = p.focal_length;
    return v;
}

double Scenario::duration() const {
    return index.back().second;
}
