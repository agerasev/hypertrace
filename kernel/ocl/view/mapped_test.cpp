#include <geometry/euclidean.hh>
#include <geometry/ray_eu.hh>
#include <transform/affine.hh>
#include "point/eu.hh"

#define $Self MapPointViewEu
#define $self map_point_view_eu
#define $Geo Eu
#define $geo eu
#define $Map Affine3
#define $map aff3
#define $View PointViewEu
#define $view point_view_eu
#include "mapped.inl"
#undef $Self
#undef $self
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $View
#undef $view
