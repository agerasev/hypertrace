#include <macros.hh>
#include <types.hh>

#if !defined($Self) || !defined($self) || !defined($elem)
#error "$Self, $self or $elem macro parameters are not defined."
#endif

$Self $2($self,_identity)();

$elem $2($self,_apply_pos)($Self m, $elem p);
$elem $2($self,_apply_dir)($Self m, $elem p, $elem d);
$elem $2($self,_apply_normal)($Self m, $elem p, $elem d);

$Self $2($self,_chain)($Self a, $Self b);
$Self $2($self,_inverse)($Self m);

#ifdef HOST
#include <iostream>

std::ostream &operator<<(std::ostream &o, $Self m);

#endif // HOST
