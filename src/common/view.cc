#include "view.hh"


void init_view(View *view) {
    view->position = mo_identity();
    view->motion = mo_identity();
    view->fov = 1.0;
}

#ifdef OPENCL_INTEROP

void pack_view(ViewPk *dst, const View *src) {
    dst->position = mo_pack(src->position);
    dst->motion = mo_pack(src->motion);
    dst->fov = src->fov;
}
void unpack_view(View *dst, const ViewPk *src) {
    dst->position = mo_unpack(src->position);
    dst->motion = mo_unpack(src->motion);
    dst->fov = src->fov;
}

#endif // OPENCL_INTEROP
