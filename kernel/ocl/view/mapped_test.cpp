#include <geometry/euclidean.hh>
#include <geometry/ray.hh>

#define $Geo Eu
#define $geo eu
#include "point.inl"
#undef $Geo
#undef $geo

#define $Self MapPointViewEu
#define $self map_point_view_eu
#define $View PointViewEu
#define $view point_view_eu
#define $Geo Eu
#define $geo eu
#include "mapped.inl"
#undef $Self
#undef $self
#undef $View
#undef $view
#undef $Geo
#undef $geo
