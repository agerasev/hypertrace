#include "variant.hpp"


#ifdef UNIT_TEST
#include <catch.hpp>
#include <test.hpp>

using namespace test;

struct A {
    int x;
    mutable float z;
    void set(float y) {
        x = int(y);
    }
    const float &get() const {
        z = float(x);
        return z;
    }
};
struct B {
    float x;
    void set(float y) {
        x = y;
    }
    const float &get() const {
        return x;
    }
};

class Var : public Variant<A, B> {
public:
    Var(Variant<A, B> v) : Variant<A, B>(v) {}
    
    template <int P, typename E>
    static Var init(const E &e) {
        return Var(Variant<A, B>::init<P>(e));
    }
    DERIVE_VARIANT_METHOD(set)
    DERIVE_VARIANT_METHOD_CONST(get)
};

TEST_CASE("Variant object", "[variant]") {
    Rng rng(0x807A);

    SECTION("Union") {
        Variant<A, B> a = Variant<A, B>::init<0>(A{1});
        Variant<A, B> b = Variant<A, B>::init<1>(B{3.1415});

        REQUIRE(a.id() == 0);
        REQUIRE(a.elem<0>().x == 1);
        REQUIRE(b.id() == 1);
        REQUIRE(b.elem<1>().x == Approx(3.1415));
    }
    SECTION("Derive") {
        Var a = Var::init<0>(A{1});
        Var b = Var::init<1>(B{3.14f});

        REQUIRE(a.get() == Approx(1));
        a.set(2.71f);
        REQUIRE(a.get() == Approx(2));

        REQUIRE(b.get() == Approx(3.14f));
        b.set(5.17f);
        REQUIRE(b.get() == Approx(5.17f));
    }
}

#endif
