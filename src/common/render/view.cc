#include "view.hh"


ViewEu vieweu_interpolate(ViewEu a, ViewEu b, real t) {
    ViewEu view;
    view.position = aff3_interpolate(a.position, b.position, t);
    view.field_of_view = a.field_of_view*(R1 - t) + b.field_of_view*t;
    view.lens_radius = a.lens_radius*(R1 - t) + b.lens_radius*t;
    view.focal_length = a.focal_length*(R1 - t) + b.focal_length*t;
    return view;
}
ViewHy viewhy_interpolate(ViewHy a, ViewHy b, real t) {
    ViewHy view;
    view.position = mo_interpolate(a.position, b.position, t);
    view.field_of_view = a.field_of_view*(R1 - t) + b.field_of_view*t;
    view.lens_radius = a.lens_radius*(R1 - t) + b.lens_radius*t;
    view.focal_length = a.focal_length*(R1 - t) + b.focal_length*t;
    return view;
}

#ifdef HOST
#ifdef INTEROP

void Interop<ViewEu>::load(Host *dst, const Dev *src) {
    dev_load(&dst->position, &src->pos);
    dst->field_of_view = src->fov;
    dst->lens_radius = src->lr;
    dst->focal_length = src->fl;
}
void Interop<ViewEu>::store(Dev *dst, const Host *src) {
    dev_store(&dst->pos, &src->position);
    dst->fov = src->field_of_view;
    dst->lr = src->lens_radius;
    dst->fl = src->focal_length;
}
void Interop<ViewHy>::load(Host *dst, const Dev *src) {
    dev_load(&dst->position, &src->pos);
    dst->field_of_view = src->fov;
    dst->lens_radius = src->lr;
    dst->focal_length = src->fl;
}
void Interop<ViewHy>::store(Dev *dst, const Host *src) {
    dev_store(&dst->pos, &src->position);
    dst->fov = src->field_of_view;
    dst->lr = src->lens_radius;
    dst->fl = src->focal_length;
}

#endif // INTEROP
#endif // HOST
