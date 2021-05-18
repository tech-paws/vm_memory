#[allow(warnings)]
#[allow(clippy::all)]
mod c_api;

use c_api::*;
use std::mem;

/// Simple region based allocator.
///
/// Allocates continuous chunk of memory with a specific size.
/// Allocator maintain a pointer within that memory, whenever allocate an object,
/// update the pointer by the object's size.
pub struct RegionAllocator {
    /// The memory reserved for the allocator.
    pub region: RegionMemoryBuffer,
}

impl RegionAllocator {
    /// Create a new allocator with a specific size.
    pub fn new(size: usize) -> Self {
        Self {
            region: unsafe { create_region_memory_buffer(size as u64) },
        }
    }

    /// Allocate a new chunk of memory with a specific size.
    /// returns the base address of the allocated chunk of memory.
    ///
    /// # Errors
    ///
    /// If the memory is run out, then this call will return an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use vm_memory::*;
    ///
    /// let mut allocator = RegionAllocator::new(1024);
    /// let base = allocator.alloc(512);
    /// assert!(base.is_ok());
    /// ```
    pub fn alloc(&mut self, size: usize) -> Result<*mut u8, &'static str> {
        let data = unsafe {
            region_memory_buffer_alloc(&mut self.region as *mut RegionMemoryBuffer, size as u64)
        };

        if data.is_null() {
            Err("Out of memory")
        } else {
            Ok(data)
        }
    }

    /// Free all memory.
    pub fn clear(&mut self) -> Result<(), &'static str> {
        unsafe { region_memory_buffer_free(&mut self.region as *mut RegionMemoryBuffer) };
        Ok(())
    }

    /// Allocate a new region of memory with size equals to size of `T` and emplace the `value`
    /// to the allocated memory.
    ///
    /// Returns a pointer to the struct located in the memory of the allocator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::mem;
    /// use vm_memory::*;
    ///
    /// let mut allocator = RegionAllocator::new(1024);
    /// let data: i32 = 12;
    /// let data: &i32 = unsafe { allocator.emplace_struct(&data).unwrap().as_ref().unwrap() };
    ///
    /// assert_eq!(mem::size_of::<i32>(), allocator.region.offset);
    /// assert_eq!(12, *data);
    /// ```
    pub fn emplace_struct<T>(&mut self, value: &T) -> Result<*mut T, &'static str> {
        let value_ptr = value as *const T;
        let data = unsafe {
            region_memory_buffer_emplace(
                &mut self.region as *mut RegionMemoryBuffer,
                mem::size_of::<T>() as u64,
                value_ptr as *const u8,
            )
        };

        if data.is_null() {
            Err("Out of memory")
        } else {
            Ok(data as *mut T)
        }
    }

    /// Allocate a new region of memory with size equals to `size` and emplace the `base`
    /// to the allocated memory.
    ///
    /// Returns a pointer to the base pointer located in the memory of the allocator.
    ///
    /// # Safety
    ///
    /// the base should point to a valid address with a valid size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::mem;
    /// use vm_memory::*;
    ///
    /// let mut allocator = RegionAllocator::new(1024);
    /// let data: i32 = 12;
    /// let data_ptr = &data as *const i32;
    /// let size = mem::size_of::<i32>() as u64;
    /// let data_emplaced_ptr = unsafe {
    ///     allocator
    ///         .emplace_buffer(data_ptr as *const u8, size)
    ///         .unwrap()
    /// };
    ///
    /// let data = unsafe { *(data_emplaced_ptr as *mut i32) };
    ///
    /// assert_eq!(size as usize, allocator.region.offset);
    /// assert_eq!(12, data);
    /// ```
    pub unsafe fn emplace_buffer(
        &mut self,
        base: *const u8,
        size: u64,
    ) -> Result<*mut u8, &'static str> {
        let data =
            region_memory_buffer_emplace(&mut self.region as *mut RegionMemoryBuffer, size, base);

        if data.is_null() {
            Err("Out of memory")
        } else {
            Ok(data)
        }
    }
}
