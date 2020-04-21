#pragma once

#include <traits.hpp>
#include <types.h>
#include <container/tuple.hpp>
#include "material.hpp"
#include <random.hpp>


template <typename ...Mats>
class Combination {
// : public Material
private:
    real probs[sizeof...(Mats)];
    Tuple<Mats...> tuple;

    template <int P=0, typename T, typename ...Args>
    void unwind(T t, Args ...args) {
        static_assert(P < sizeof...(Mats), "Too many arguments");
        probs[P] = t.first;
        tuple.template elem<P>() = t.second;
        unwind<P + 1>(args...);
    }
    template <int P=0>
    void unwind() {
        static_assert(P == sizeof...(Mats), "Too few arguments");
    }

    void normalize_probs() {
        real a = 0_r;
        for (int i = 0; i < int(sizeof...(Mats)); ++i) {
            a += probs[i];
        }
        for (int i = 0; i < int(sizeof...(Mats)); ++i) {
            probs[i] /= a;
        }
    }

public:
    Combination() = default;
    template <typename ...Args>
    Combination(Args ...args) {
        unwind(args...);
        normalize_probs();
    }
    real *portions() {
        return probs;
    }
    const real *portions() const {
        return probs;
    }
    Tuple<Mats...> &as_tuple() {
        return tuple;
    }
    const Tuple<Mats...> &as_tuple() const {
        return tuple;
    }

private:
    template <typename E, int P>
    struct InteractCaller {
        template <typename Context>
        static real call(
            const E &e,
            Context &context, real3 normal,
            LocalLight &light, float3 &emission
        ) {
            return e.interact(context, normal, light, emission);
        }
    };
public:
    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        real a = xrand::uniform<real>(context.rng);
        int i = 0;
        for (; i < tuple.size(); ++i) {
            a -= probs[i];
            if (a <= 0_r) {
                break;
            }
        }
        return tuple.template call<InteractCaller, 0>(
            i, context, normal, light, emission
        );
    }
};

#ifdef HOST

template <int N, typename ...Mats>
struct _CombinationPacker {
    static void pack(Tuple<device_type<Mats>...> &dst, const Tuple<Mats...> &src) {
        static const int P = sizeof...(Mats) - N;
        dst.template elem<P>() = ::to_device(src.template elem<P>());
        _CombinationPacker<N - 1, Mats...>::pack(dst, src);
    }
};
template <typename ...Mats>
struct _CombinationPacker<0, Mats...> {
    static void pack(Tuple<device_type<Mats>...> &dst, const Tuple<Mats...> &src) {}
};

template <typename ...Mats>
struct ToDevice<Combination<Mats...>> {
public:
    struct type {
        device::real probs[sizeof...(Mats)];
        Tuple<device_type<Mats>...> tuple;
    };
public:
    static type to_device(const Combination<Mats...> &mat) {
        type out;
        for (int i = 0; i < int(sizeof...(Mats)); ++i) {
            out.probs[i] = ::to_device(mat.portions()[i]);
        }
        _CombinationPacker<sizeof...(Mats), Mats...>::pack(out.tuple, mat.as_tuple());
        return out;
    }
};

#endif
