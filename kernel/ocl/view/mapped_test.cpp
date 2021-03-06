#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <transform/affine.hh>
#include "point/eu.hh"

typedef struct {
    PointViewEu inner;
    Affine3 map;
} Mapped;

#define $Self ViewMapped
#define $self view_mapped
#define $Base Mapped
#define $base mapped
#define $Geo Eu
#define $geo eu
#define $Map Affine3
#define $map aff3
#define $View PointViewEu
#define $view point_view_eu
#include "mapped.inl"
#undef $Self
#undef $self
#undef $Base
#undef $base
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $View
#undef $view
