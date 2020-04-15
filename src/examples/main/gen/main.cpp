#include <iostream>
#include <fstream>
#include <cassert>

int main(int argc, char *argv[]) {
    assert(argc == 1 + 1);
    
    int path_max_depth = 6;
    int path_max_diffuse_depth = 2;

    // congif.h
    std::ofstream cfg(argv[1]);
    assert(cfg.good());
    cfg << "#pragma once\n\n"
        << "#define PATH_MAX_DEPTH " << path_max_depth << "\n"
        << "#define PATH_MAX_DIFFUSE_DEPTH " << path_max_diffuse_depth << "\n"
        << std::endl;
    cfg.close();

    return 0;
}
