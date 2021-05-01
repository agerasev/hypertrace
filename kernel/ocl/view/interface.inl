#include <macros.hh>
#include <types.hh>
#include <render/context.hh>

#if !defined($Geo) || !defined($geo) || !defined($Self) || !defined($self)
#error "All required macro parameters must be defined."
#endif

$2(Ray,$Geo) $2($self,_sample)(__global const $Self *view, Context *context, real2 pixel_pos, real2 pixel_size);
