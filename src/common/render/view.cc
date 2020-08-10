#include "view.hh"


ViewEu vieweu_interpolate(ViewEu a, ViewEu b, real t) {
    return ViewEu {
        .position = aff3_interpolate(a.position, b.position, t),
        .field_of_view = a.field_of_view*(R1 - t) + b.field_of_view*t,
        .lens_radius = a.lens_radius*(R1 - t) + b.lens_radius*t,
        .focal_length = a.focal_length*(R1 - t) + b.focal_length*t,
    };
}

ViewHy viewhy_interpolate(ViewHy a, ViewHy b, real t) {
    return ViewHy {
        .position = mo_interpolate(a.position, b.position, t),
        .field_of_view = a.field_of_view*(R1 - t) + b.field_of_view*t,
        .lens_radius = a.lens_radius*(R1 - t) + b.lens_radius*t,
        .focal_length = a.focal_length*(R1 - t) + b.focal_length*t,
    };
}
