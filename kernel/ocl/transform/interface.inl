#include <macros.hh>
#include <types.hh>

#if !defined($Map) || !defined($map) || !defined($elem)
#error "$Map, $map or $elem macro parameters are not defined."
#endif

$Map $2($map,_identity)();

$elem $2($map,_apply_pos)($Map m, $elem p);
$elem $2($map,_apply_dir)($Map m, $elem p, $elem d);
$elem $2($map,_apply_normal)($Map m, $elem p, $elem d);

$Map $2($map,_chain)($Map a, $Map b);
$Map $2($map,_inverse)($Map m);

#ifdef HOST
#include <iostream>

std::ostream &operator<<(std::ostream &o, $Map m);

#endif // HOST
