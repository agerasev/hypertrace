#include <macros.hh>
#include <algebra/color.hh>
#include <render/context.hh>
#include <render/light/base.hh>

#if !defined($Self)  || !defined($self)
#error "All required macro parameters must be defined."
#endif

void $2($self,_interact)(__global const $Self *self, Context *context, const LightLocal *light, color3 *emission);
