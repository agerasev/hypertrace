#include "scenario.hpp"

#include <memory>
#include <vector>
#include <cassert>
#include <algorithm>

#include <view.hh>


Transition::Transition(double d) :
    duration(d)
{}

DelayTransition::DelayTransition(double d, View p) :
    Transition(d), view(p)
{}
View DelayTransition::get_view(double p) const {
    return view;
}

LinearTransition::LinearTransition(double d, View a, View b) :
    Transition(d), start(a), stop(b)
{}
View LinearTransition::get_view(double p) const {
    return view_interpolate(start, stop, p);
}

SquareTransition::SquareTransition(double d, View a, View b, double at, double bt) :
    LinearTransition(d, a, b), markers{at, bt}
{}
View SquareTransition::get_view(double p) const {
    return view_interpolate(
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

View PathScenario::get_view(double time) const {
    assert(index.size() > 0);

    static const auto comp = [](
        const std::pair<std::unique_ptr<Transition>, double> &a,
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
