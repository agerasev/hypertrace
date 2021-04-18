#include <macros.hh>
#include <types.hh>
#include <geometry/geometry.hh>

#if !defined($Geo) || !defined($geo)
#error "$Geo and $geo macros must be defined."
#endif

typedef struct {
    $2($Geo,Map) position;
    real field_of_view;
    real lens_radius;
    real focal_length;
} $2(View,$Geo);

//#ifndef HOST
//#include "view.cc"
//#endif // !HOST
