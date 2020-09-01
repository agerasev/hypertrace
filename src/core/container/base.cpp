#include "base.hpp"

#ifdef TEST_UNIT

#include <memory>
#include <catch.hpp>

using namespace container;

template <typename T>
class Wrap final : public CopyEnabler<is_copyable_v<T>> {
private:
    T t;
public:
    Wrap() = delete;
    
    Wrap(const T &t) : t(t) {}
    Wrap(T &&t) : t(std::move(t)) {}

    Wrap(const Wrap &w) = default;
    Wrap(Wrap &&w) = default;

    Wrap &operator=(const Wrap &w) = default;
    Wrap &operator=(Wrap &&w) = default;

    T &operator*() { return t; }
    const T &operator*() const { return t; }
};

TEST_CASE("Base", "[base]") {
    SECTION("Copyable") {
        Wrap<int> w(3);
        REQUIRE(*w == 3);
        Wrap<int> cw(w);
        REQUIRE(*cw == 3);
    }
    SECTION("Movable") {
        Wrap<std::unique_ptr<int>> w(std::make_unique<int>(3));
        REQUIRE(**w == 3);
    }
    SECTION("Nested") {
        Wrap<Wrap<std::unique_ptr<int>>> w(Wrap<std::unique_ptr<int>>(std::make_unique<int>(3)));
        REQUIRE(***w == 3);
    }
}

#endif // TEST_UNIT
