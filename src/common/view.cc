#include "view.hh"


View view_init() {
    View v;
    v.position = mo_identity();
    v.field_of_view = (real)1.0f;
    v.lens_radius = (real)1e-2f;
    v.focal_length = (real)5.0f;
    return v;
}

View view_position(Moebius m) {
    View v = view_init();
    v.position = m;
    return v;
}

View view_interpolate(View a, View b, real t) {
    View o;
    o.position = mo_chain(
        a.position,
        mo_pow(mo_chain(mo_inverse(a.position), b.position), t)
    );
    o.field_of_view = a.field_of_view*(1 - t) + b.field_of_view*t;
    o.lens_radius = a.lens_radius*(1 - t) + b.lens_radius*t;
    o.focal_length = a.focal_length*(1 - t) + b.focal_length*t;
    return o;
}

#ifdef OPENCL_INTEROP

ViewPk view_pack(View v) {
    ViewPk o;
    o.position = mo_pack(v.position);
    o.field_of_view = v.field_of_view;
    o.lens_radius = v.lens_radius;
    o.focal_length = v.focal_length;
    return o;
}
View view_unpack(ViewPk v) {
    View o;
    o.position = mo_unpack(v.position);
    o.field_of_view = v.field_of_view;
    o.lens_radius = v.lens_radius;
    o.focal_length = v.focal_length;
    return o;
}

#endif // OPENCL_INTEROP
