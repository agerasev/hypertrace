#include "view.hh"


void init_view(View *view) {
    view->position = mo_identity();
    view->motion = mo_identity();
    view->field_of_view = (real)1.0f;
    view->lens_radius = (real)1e-2f;
    view->focal_length = (real)5.0f;
}

#ifdef OPENCL_INTEROP

void pack_view(ViewPk *dst, const View *src) {
    dst->position = mo_pack(src->position);
    dst->motion = mo_pack(src->motion);
    dst->field_of_view = src->field_of_view;
    dst->lens_radius = src->lens_radius;
    dst->focal_length = src->focal_length;
}
void unpack_view(View *dst, const ViewPk *src) {
    dst->position = mo_unpack(src->position);
    dst->motion = mo_unpack(src->motion);
    dst->field_of_view = src->field_of_view;
    dst->lens_radius = src->lens_radius;
    dst->focal_length = src->focal_length;
}

#endif // OPENCL_INTEROP
