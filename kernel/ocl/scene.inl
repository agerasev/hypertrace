#include <macros.hh>
#include <geometry/geometry.hh>

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Geo)   || !defined($geo) || \
    !defined($View)   || !defined($view) || \
    !defined($Object) || !defined($object)
#error "All required macro parameters must be defined."
#endif

real $2($shape,_detect)(__global const $Self *self, Context *context) {
    
}
