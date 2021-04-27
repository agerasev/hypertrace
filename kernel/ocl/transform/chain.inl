#include <macros.hh>
#include <types.hh>

#if \
    !defined($Self) || !defined($self) || \
    !defined($Outer) || !defined($outer) || \
    !defined($Inner) || !defined($inner) || \
    !defined($elem)
#error "Some of macro parameters are not defined."
#endif

#ifndef CHAIN_NO_DEF
typedef struct {
    $Outer outer;
    $Inner inner;
} $Self;
#endif // CHAIN_NO_DEF

#define $Map $Self
#define $map $self
#include "derive.inl"
#undef $Map
#undef $map

$Self $2($self,_identity)() {
    return $Self {
        $2($inner,_identity)(),
        $2($outer,_identity)()
    }
}

$elem $2($self,_apply_pos)($Self m, $elem p) {
    return $2($outer,_apply_pos)(m.outer, $2($inner,_apply_pos)(m.inner, v));
}
$elem $2($self,_apply_dir)($Self m, $elem p, $elem d) {
    return $2($outer,_apply_dir)(
        m.outer,
        $2($inner,_apply_pos)(m.inner, p),
        $2($inner,_apply_dir)(m.inner, p, d)
    );
}
$elem $2($self,_apply_normal)($Self m, $elem p, $elem d) {
    return $2($outer,_apply_normal)(
        m.outer,
        $2($inner,_apply_pos)(m.inner, p),
        $2($inner,_apply_normal)(m.inner, p, d)
    );
}

$Self $2($self,_chain)($Self a, $Self b) {
    $4($inner,_,$outer,_reorder)(&a.inner, &b.outer);
    return $Self {
        $2($outer,_chain)(a.outer, b.outer),
        $2($inner,_chain)(a.inner, b.inner)
    }
}
$Self $2($self,_inverse)($Self m) {
    $Self s = $Self {
        $2($outer,_inverse)(m.outer),
        $2($inner,_inverse)(m.inner)
    };
    $4($inner,_,$outer,_reorder)(&s.inner, &s.outer);
    return s;
}

real $2($self,_distance)($Self a, $Self b) {
    real ad = $2($outer,_distance)(a);
    real bd = $2($inner,_distance)(b);
    return ad + bd;
}
