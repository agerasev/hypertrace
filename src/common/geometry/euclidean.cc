#include "euclidean.hh"
#include <common/algebra/rotation.hh>


EuPos eu_origin() {
    return MAKE(real3)(0);
}

real eu_length(EuPos a) {
    return length(a);
}
real eu_distance(EuPos a, EuPos b) {
    return length(a - b);
}

EuPos eu_apply_pos(EuMap m, EuPos p) {
    return aff3_apply_pos(m, p);
}
#pragma GCC diagnostic ignored "-Wunused-parameter"
EuDir eu_apply_dir(EuMap m, EuPos p, EuDir d) {
    return aff3_apply_dir(m, d);
}

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
#pragma GCC diagnostic ignored "-Wunused-parameter"
EuDir eu_dir_at(EuPos src_pos, EuDir src_dir, EuPos dst_pos) {
    return src_dir;
}

EuMap eu_shift(EuDir pos) {
    return aff3_from_ls(lin3_identity(), pos);
}
EuMap eu_rotate(EuDir axis, real phi) {
    return rot3_to_linear(rot3_from_axis(axis, phi));
}

// Turns direction `dir` to *z*.
EuMap eu_look_to(EuDir dir) {
    return lin3_look_to(dir);
}

// Rotates point `pos` around the origin to make it lay on the z axis.
EuMap eu_look_at(EuPos pos) {
    return eu_look_to(normalize(pos));
}

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
EuMap eu_move_at(EuPos pos) {
    return aff3_from_ls(lin3_identity(), -pos);
}
EuMap eu_move_to(EuDir dir, real dist) {
    return eu_move_at(dir*dist);
}


#ifdef TEST

#include <rtest.hpp>

#ifdef TEST_UNIT

rtest_module_(euclidean) {
    static_thread_local_(TestRng<real3>, vrng) {
        return TestRng<real3>();
    }
    static_thread_local_(TestRngRotation3, rot3rng) {
        return TestRngRotation3();
    }

    rtest_(distance_invariance) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 a = vrng->normal(), b = vrng->normal();

            Affine3 m = aff3_from_ls(
                rot3_to_linear(rot3rng->uniform()),
                vrng->normal()
            );

            real dist_before = eu_distance(a, b);
            real dist_after = eu_distance(
                eu_apply_pos(m, a),
                eu_apply_pos(m, b)
            );

            assert_eq_(dist_before, approx(dist_after));
        }
    }
    rtest_(look_at_the_point) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 q = vrng->normal();
            real3 p = eu_apply_pos(eu_look_at(q), q);

            assert_eq_(p.xy, approx(r2_new(R0, R0)));
        }
    }
    rtest_(move_at_the_point) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 p = vrng->normal(), q = vrng->normal();

            Affine3 a = eu_move_at(p);
            assert_eq_(eu_apply_pos(a, p), approx(eu_origin()));

            Affine3 b = eu_chain(eu_inverse(eu_move_at(q)), a);
            assert_eq_(eu_apply_pos(b, p), approx(q));
        }
    }
};

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern_lazy_static_(devtest::Selector, devtest_selector);

rtest_module_(euclidean) {
    rtest_(distance_invariance) {
        TestRng<real3> vrng(0xdead);
        TestRngRotation3 rrng(0xbeef);

        for (devtest::Target target : *devtest_selector) {
            auto queue = target.make_queue();
            auto kernel = devtest::KernelBuilder(target.device_id(), queue)
            .source("euclidean.cl", std::string(
                "#include <common/geometry/euclidean.hh>\n"
                "__kernel void transform(__global const EuMap *map, __global const EuPos *ipos, __global EuPos *opos) {\n"
                "    int i = get_global_id(0);\n"
                "    opos[i] = eu_apply_pos(map[i/2], ipos[i]);\n"
                "}\n"
            ))
            .build("transform").unwrap();

            const int n = TEST_ATTEMPTS;
            std::vector<Eu::Pos> ipos(2*n), opos(2*n);
            std::vector<Eu::Map> map(n);
            std::vector<real> dist(n);
            for (size_t i = 0; i < n; ++i) {
                Eu::Pos x = vrng.normal(), y = vrng.normal();
                ipos[2*i] = x;
                ipos[2*i + 1] = y;
                map[i] = aff3_from_ls(rot3_to_linear(rrng.uniform()), vrng.normal());
                dist[i] = Eu::distance(x, y);
            }

            devtest::KernelRunner(queue, std::move(kernel))
            .run(2*n, map, ipos, opos).expect("Kernel run error");

            for(size_t i = 0; i < n; ++i) {
                Eu::Pos x = opos[2*i], y = opos[2*i + 1];
                assert_eq_(Eu::distance(x, y), approx(dist[i]).epsilon(1e-4));
            }
        }
    }
}

#endif // TEST_DEV

#endif // TEST
