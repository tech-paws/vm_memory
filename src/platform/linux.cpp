#include "vm_memory.hpp"
#include <sys/mman.h>
#include <fcntl.h>

extern "C" uint8_t* virtual_alloc(uint32_t size) {
    void* base = mmap(0, size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);

    if (base != MAP_FAILED) {
        memset(base, 0, size);
        return (uint8_t*) base;
    }
    else {
        return 0;
    }
}
