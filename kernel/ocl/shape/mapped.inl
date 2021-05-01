#include <macros.hh>
#include <geometry/geometry.hh>

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Geo)   || !defined($geo) || \
    !defined($Shape) || !defined($shape) || \
    !defined($Map)   || !defined($map)
#error "All required macro parameters must be defined."
#endif

real $2($shape,_detect)(__global const $Self *self, Context *context, $2($Geo,Pos) *normal, $2(Ray,$Geo) *ray) {
    
}
