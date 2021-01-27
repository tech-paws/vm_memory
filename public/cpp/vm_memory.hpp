#include <stdint.h>
#include <string.h>

struct RegionMemoryBuffer {
    uint64_t size;
    uint8_t* base;
    uintptr_t offset;
};

struct StackMemoryBuffer {
    uint64_t size;
    uint8_t* base;
    uintptr_t offset;
};

extern "C" uint8_t* virtual_alloc(uint32_t size);

extern "C" RegionMemoryBuffer create_region_memory_buffer(uint64_t size);

extern "C" RegionMemoryBuffer region_memory_buffer_emplace_region(RegionMemoryBuffer* where, uint64_t size);

extern "C" uint8_t* region_memory_buffer_alloc(RegionMemoryBuffer* buffer, uint64_t size);

extern "C" uint8_t* region_memory_buffer_emplace(RegionMemoryBuffer* buffer, uint64_t size, uint8_t const* data);

extern "C" void region_memory_buffer_free(RegionMemoryBuffer* buffer);
