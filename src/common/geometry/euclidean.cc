#include "euclidean.hh"
#include <common/algebra/rotation.hh>


EuPos eu_origin() {
    return real3(0);
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
EuDir eu_apply_dir(EuMap m, EuPos, EuDir d) {
    return aff3_apply_dir(m, d);
}

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
EuDir eu_dir_at(EuPos, EuDir src_dir, EuPos) {
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


#ifdef TEST_UNIT

#include <rtest.hpp>


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
