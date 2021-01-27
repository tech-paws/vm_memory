#include "vm_memory.hpp"
#include <assert.h>

extern "C" RegionMemoryBuffer create_region_memory_buffer(uint64_t size) {
    uint8_t* base = virtual_alloc(size);
    RegionMemoryBuffer buffer;

    if (base) {
        buffer.size = size;
        buffer.base = base;
        buffer.offset = 0;
    }
    else {
        buffer.size = 0;
        buffer.base = 0;
        buffer.offset = 0;
    }

    return buffer;
}

extern "C" RegionMemoryBuffer region_memory_buffer_emplace_region(RegionMemoryBuffer* where, uint64_t size) {
    assert(where->offset + size <= where->size);
    RegionMemoryBuffer buffer;

    buffer.base = where->base + where->offset;
    buffer.size = size;
    buffer.offset = 0;

    where->offset += size;

    return buffer;
}

extern "C" uint8_t* region_memory_buffer_alloc(RegionMemoryBuffer* buffer, uint64_t size) {
    assert(buffer != 0);

    if (buffer->offset + size > buffer->size) {
        return 0;
    }

    uint8_t* result = buffer->base + buffer->offset;
    buffer->offset += size;

    return result;
}

extern "C" uint8_t* region_memory_buffer_emplace(RegionMemoryBuffer* buffer, uint64_t size, uint8_t const* data) {
    uint8_t* result = region_memory_buffer_alloc(buffer, size);
    memcpy(result, data, size);
    return result;
}

extern "C" void region_memory_buffer_free(RegionMemoryBuffer* buffer) {
    buffer->offset = 0;
}
