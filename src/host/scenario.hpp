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

    void set_start_point(const Point &p) {
        assert(!start && !stop);
        start = std::make_shared<PointNode>(p);
        stop = start;
        index.push_back(std::make_pair(start, 0.0));
    }

    public:
    void add_transition(const Transition &t, const Point &p) {
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
    template <typename ... Args>
    void add_transition(const Transition &t, const Point &p, const Args &...args) {
        add_transition(t, p);
        add_transition(args...);
    }

    public:
    Scenario(const Point &p) {
        set_start_point(p);
    }
    template <typename ... Args>
    Scenario(const Point &p, const Args &...args) : Scenario(p) {
        add_transition(args...);
    }

    Point get_point(double time) const {
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
    View get_view(double t, double dt) const {
        Point p = get_point(t);
        View v;
        init_view(&v);
        v.position = p.position;
        v.focal_length = p.focal_length;
        return v;
    }
    double duration() const {
        return index.back().second;
    }
};
