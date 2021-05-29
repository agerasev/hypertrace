#pragma once

#include <macros.hh>
#include <types.hh>

#define __define_transform_interface($Self, $self, $elem) \
    $Self $2($self,_identity)(); \
 \
    $elem $2($self,_apply_pos)($Self m, $elem p); \
    $elem $2($self,_apply_dir)($Self m, $elem p, $elem d); \
    $elem $2($self,_apply_normal)($Self m, $elem p, $elem d); \
 \
    $Self $2($self,_chain)($Self a, $Self b); \
    $Self $2($self,_inverse)($Self m); \

#ifdef HOST
#include <iostream>

#define define_transform_interface($Self, $self, $elem) \
    __define_transform_interface($Self, $self, $elem) \
    std::ostream &operator<<(std::ostream &o, $Self m);

#else // !HOST

#define define_transform_interface($Self, $self, $elem) \
    __define_transform_interface($Self, $self, $elem)

#endif // HOST
