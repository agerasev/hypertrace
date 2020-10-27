#include <atomic>
#include <rtest/main.hpp>
#include <ansi_color.hpp>

#include "devtest.hpp"

using namespace devtest;


std::atomic<const Device *> __devtest_device;
Target devtest_make_target() {
    return __devtest_device.load()->make_target();
}

int main(int argc, char *argv[]) {
    Machine machine;
    size_t passed = 0, failed = 0, skipped = 0;
    for (const Platform &platform : machine.platforms()) {
        println_("Testing platform:\n{}", platform.info());
        
        size_t device_count = platform.devices().size();
        println_("Total devices: {}", device_count);
        if (device_count == 0) {
            println_("No device to run tests");
            skipped += 1;
            continue;
        }
        const Device &device = platform.devices()[0];
        println_("Testing first device:\n{}", device.info());

        __devtest_device.store(&device);
        if (rtest::main(argc, argv) == 0) {
            passed += 1;
        } else {
            failed += 1;
        }
        println_();
    }   

#ifndef RTEST_BW
    std::string result_name[2] = {
        ansi_color("ok", AnsiColor::FG_GREEN),
        ansi_color("FAILED", AnsiColor::FG_RED, AnsiColor::BOLD)
    };
#else // RTEST_BW
    std::string result_name[2] = {"ok", "FAILED"};
#endif // RTEST_BW
    println_("Total result by platforms: {}. {} passed; {} failed; {} skipped;", result_name[failed != 0], passed, failed, skipped);
    println_();

    return failed;
}
