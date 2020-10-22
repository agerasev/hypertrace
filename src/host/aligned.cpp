#include <cstdlib>
#include <memory>
#include <common/math.hh>


#if defined(__MINGW32__) || defined(__MINGW64__)
// There is no std::aligned_alloc in MinGW
void *aligned_alloc_(size_t align, size_t size, void **orig) {
    void *mem = std::malloc(size + align - 1);
    *orig = mem;
    return (void*)upper_multiple(align, (size_t)mem);
}
#else
void *aligned_alloc_(size_t align, size_t size, void **orig) {
    void *mem = std::aligned_alloc(align, size);
    *orig = mem;
    return mem;
}
#endif
