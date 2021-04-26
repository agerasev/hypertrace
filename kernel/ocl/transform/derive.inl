#include <macros.hh>

#if !defined($Map)  || !defined($map) || !defined($elem)
#error "$Map, $map or $Space macro parameters are not defined."
#endif

$Map $2($map,_identity)();

$elem $2($map,_apply_pos)($Map m, $elem v);
$elem $2($map,_apply_dir)($Map m, $elem p, $elem d);
$elem $2($map,_apply_normal)($Map m, $elem p, $elem d);

$Map $2($map,_chain)($Map a, $Map b);
$Map $2($map,_inverse)($Map m);

real $2($map,_distance)($Map a, $Map b);

#ifdef HOST
#include <iostream>

inline std::ostream &operator<<(std::ostream &o, $Map m) {
    return o << STRINGIFY($Map) << " { " << m.v << " }";
}

template <>
class Approx<$Map> {
public:
    typedef $Map M;
    real _epsilon = APPROX_EPS;
    M _value;

    explicit Approx(M value) :
        _value(value)    
    {}
    Approx epsilon(real eps) const {
        Approx copy_ = *this;
        copy_._epsilon = eps;
        return copy_;
    }
    bool operator==(M m) const {
        return $2($map,_distance)(_value, m) <= _epsilon;
    }
};

#endif // HOST
