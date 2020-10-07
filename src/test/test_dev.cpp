#include <lazy_static.hpp>
#include <rtest/main.hpp>

#include "devtest.hpp"


lazy_static_(devtest::Selector, devtest_selector) {
    return devtest::Selector();
}

int main(int argc, char *argv[]) {
    devtest_selector->print_info();
    return rtest::main(argc, argv);
}
