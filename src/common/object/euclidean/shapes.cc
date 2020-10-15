#include "shapes.hh"


real sphereeu_detect(Context *context, real3 *normal, LightEu *light) {
    real3 pos = light->ray.start;
    real3 dir = light->ray.direction;
    real b = -dot(dir, pos);
    real c = dot(pos, pos) - R1;
    real d = b*b - c;
    if (d < R0) {
        return -R1;
    }
    d = sqrt(d);
    real e = b - d;
    if (e < EPS*context->repeat) {
        return -R1;
    }
    real3 h = pos + dir*e;
    *normal = h;
    light->ray.start = h;
    return e;
}

static real cubeeu_detect_nearest(real3 near, real3 *normal) {
    bool xy = near.x > near.y;
    bool yz = near.y > near.z;
    bool xz = near.x > near.z;
    real dist = R0;
    //*normal = MAKE(real3)(R0);
    if (xy && xz) {
        dist = near.x;
        normal->x = R1;
    } else if (yz) {
        dist = near.y;
        normal->y = R1;
    } else {
        dist = near.z;
        normal->z = R1;
    }
    return dist;
}

real cubeeu_detect(Context *context, real3 *normal, LightEu *light) {
    const real3 cmax = MAKE(real3)(R1);
    const real3 cmin = MAKE(real3)(-R1);

    real3 inv_dir = R1/light->ray.direction;

    real3 vmin = (cmin - light->ray.start)*inv_dir;
    real3 vmax = (cmax - light->ray.start)*inv_dir;

    real3 near = fmin(vmin, vmax);
    real3 far = fmax(vmin, vmax);

    real3 norm_in = MAKE(real3)(R0);
    float dist_in = cubeeu_detect_nearest(near, &norm_in);
    norm_in *= -sign(light->ray.direction);

    real3 norm_out = MAKE(real3)(R0);
    float dist_out = -cubeeu_detect_nearest(-far, &norm_out);
    norm_out *= sign(light->ray.direction);

    if (dist_in < EPS*context->repeat || dist_in > dist_out) {
        return -R1;
    }

    light->ray.start += dist_in*light->ray.direction;
    *normal = norm_in;
    return dist_in;
}


#ifdef TEST

#include <rtest.hpp>

real max_comp(real3 v) {
    return max(max(v.x, v.y), v.z);
}
real max_comp_idx(real3 v) {
    if (v.y > v.x) {
        if (v.z > v.y) {
            return 2;
        } else {
            return 1;
        }
    } else {
        if (v.z > v.x) {
            return 2;
        } else {
            return 0;
        }
    }
}

#ifdef TEST_UNIT

rtest_module_(euclidean_shapes) {
    static_thread_local_(TestRng<real3>, vrng) {
        return TestRng<real3>(0xBAAB);
    }

    rtest_(sphere) {
        Context ctx;
        ctx.repeat = false;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 start = vrng->normal(), dir = vrng->unit();
            LightEu light;
            light.ray = RayEu { start, dir };
            real3 normal;
            
            real dist = sphereeu_detect(&ctx, &normal, &light);

            if (length(start) < 1 - EPS) {
                // TODO: Update on refraction
                assert_eq_(dist, approx(-1));
            }
            if (length(start) > 1 + EPS) {
                real proj = -dot(start, dir);
                if (proj > EPS && length(start + proj*dir) < 1 - EPS) {
                    assert_eq_(length2(light.ray.start), approx(1));
                    assert_eq_(light.ray.start, approx(normal));
                    assert_(dist > -EPS);
                } else {
                    assert_eq_(dist, approx(-1));
                }
            }
        }
    }
    rtest_(cube) {
        Context ctx;
        ctx.repeat = false;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 start = vrng->normal(), dir = vrng->unit();
            LightEu light;
            light.ray = RayEu { start, dir };
            real3 normal;

            real dist = cubeeu_detect(&ctx, &normal, &light);
            
            if (max_comp(fabs(start)) < 1 - EPS) {
                // TODO: Update on refraction
                assert_eq_(dist, approx(-1));
            }
            if (max_comp(fabs(start)) > 1 + EPS) {
                real proj = -dot(start, dir);
                bool sph = proj > EPS && length(start + proj*dir) < 1 - EPS;
                if (sph || dist > EPS) {
                    assert_eq_(max_comp(fabs(light.ray.start)), approx(1));
                    int idx = max_comp_idx(fabs(light.ray.start));
                    assert_eq_(normal[idx], approx(sign(light.ray.start[idx])));
                    normal[idx] = 0;
                    assert_eq_(normal, approx(real3(0)));
                    assert_(dist > -EPS);
                }
            }
        }
    }
};

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern devtest::Target devtest_make_target();

rtest_module_(euclidean_shapes) {
    rtest_(sphere_detect) {
        TestRng<real3> vrng(0xBAAB);
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("euclidean_sphere.cl", std::string(
            "#include <common/object/euclidean/shapes.hh>\n"
            "__kernel void detect(\n"
            "    __global const real3 *start, __global const real3 *dir,\n"
            "    __global real3 *hit, __global real3 *norm, __global real *dist\n"
            ") {\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    ctx.repeat = false;\n"
            "    LightEu light;\n"
            "    light.ray.start = start[i];\n"
            "    light.ray.direction = dir[i];\n"
            "    real3 normal;\n"
            "    dist[i] = sphereeu_detect(&ctx, &normal, &light);\n"
            "    hit[i] = light.ray.start;\n"
            "    norm[i] = normal;\n"
            "}\n"
        ))
        .build("detect").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<real3> start(n), dir(n), hit(n), norm(n);
        std::vector<real> dist(n);
        for (size_t i = 0; i < n; ++i) {
            start[i] = vrng.normal();
            dir[i] = vrng.unit();
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, start, dir, hit, norm, dist).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            if (dist[i] > -DEV_EPS) {
                assert_eq_(length(hit[i]), dev_approx(1));
                assert_eq_(hit[i], dev_approx(norm[i]));
            } else {
                assert_eq_(dist[i], dev_approx(-1));
            }
        }
    }
    rtest_(cube_detect) {
        TestRng<real3> vrng(0xBAAB);
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("euclidean_cube.cl", std::string(
            "#include <common/object/euclidean/shapes.hh>\n"
            "__kernel void detect(\n"
            "    __global const real3 *start, __global const real3 *dir,\n"
            "    __global real3 *hit, __global real3 *norm, __global real *dist\n"
            ") {\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    ctx.repeat = false;\n"
            "    LightEu light;\n"
            "    light.ray.start = start[i];\n"
            "    light.ray.direction = dir[i];\n"
            "    real3 normal;\n"
            "    dist[i] = cubeeu_detect(&ctx, &normal, &light);\n"
            "    hit[i] = light.ray.start;\n"
            "    norm[i] = normal;\n"
            "}\n"
        ))
        .build("detect").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<real3> start(n), dir(n), hit(n), norm(n);
        std::vector<real> dist(n);
        for (size_t i = 0; i < n; ++i) {
            start[i] = vrng.normal();
            dir[i] = vrng.unit();
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, start, dir, hit, norm, dist).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            if (dist[i] > -DEV_EPS) {
                assert_eq_(max_comp(fabs(hit[i])), dev_approx(1));
                int idx = max_comp_idx(fabs(hit[i]));
                assert_eq_(norm[i][idx], dev_approx(sign(hit[i][idx])));
                norm[i][idx] = 0;
                assert_eq_(norm[i], dev_approx(real3(0)));
            } else {
                assert_eq_(dist[i], dev_approx(-1));
            }
        }
    }
}

#endif // TEST_DEV

#endif // TEST
