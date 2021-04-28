#include <macros.hh>
#include <types.hh>

#if \
    !defined($Self) || !defined($self) || \
    !defined($Outer) || !defined($outer) || \
    !defined($Inner) || !defined($inner) || \
    !defined($elem)
#error "Some of macro parameters are not defined."
#endif

typedef struct {
    $Outer outer;
    $Inner inner;
} $Self;

#define $Map $Self
#define $map $self
#include "interface.inl"
#undef $Map
#undef $map

_ALLOW_MULTIPLE_DEFINITIONS_
$Self $2($self,_identity)() {
    return $Self {
        $2($outer,_identity)(),
        $2($inner,_identity)()
    };
}

_ALLOW_MULTIPLE_DEFINITIONS_
$elem $2($self,_apply_pos)($Self m, $elem p) {
    return $2($outer,_apply_pos)(m.outer, $2($inner,_apply_pos)(m.inner, p));
}
_ALLOW_MULTIPLE_DEFINITIONS_
$elem $2($self,_apply_dir)($Self m, $elem p, $elem d) {
    return $2($outer,_apply_dir)(
        m.outer,
        $2($inner,_apply_pos)(m.inner, p),
        $2($inner,_apply_dir)(m.inner, p, d)
    );
}
_ALLOW_MULTIPLE_DEFINITIONS_
$elem $2($self,_apply_normal)($Self m, $elem p, $elem d) {
    return $2($outer,_apply_normal)(
        m.outer,
        $2($inner,_apply_pos)(m.inner, p),
        $2($inner,_apply_normal)(m.inner, p, d)
    );
}

_ALLOW_MULTIPLE_DEFINITIONS_
$Self $2($self,_chain)($Self a, $Self b) {
    $4($inner,_,$outer,_reorder)(&a.inner, &b.outer);
    return $Self {
        $2($outer,_chain)(a.outer, b.outer),
        $2($inner,_chain)(a.inner, b.inner)
    };
}
_ALLOW_MULTIPLE_DEFINITIONS_
$Self $2($self,_inverse)($Self m) {
    $Self s = $Self {
        $2($outer,_inverse)(m.outer),
        $2($inner,_inverse)(m.inner)
    };
    $4($inner,_,$outer,_reorder)(&s.inner, &s.outer);
    return s;
}

#ifdef HOST

template <>
struct Distance<$Self> {
    inline static real distance($Self a, $Self b) {
        real ad = Distance<$Outer>::distance(a.outer, b.outer);
        real bd = Distance<$Inner>::distance(a.inner, b.inner);
        return ad + bd;
    }
};

inline std::ostream &operator<<(std::ostream &o, $Self m) {
    return o << "Chain { " << m.outer << ", " << m.inner << " }";
}

#endif // HOST
