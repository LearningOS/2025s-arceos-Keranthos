#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator, AllocError, AllocResult};
use core::alloc::Layout;
use core::ptr::NonNull;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize, 
    end: usize, 
    b_pos: usize,
    p_pos: usize,
    bytes_count: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            bytes_count: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = start; 
        self.p_pos = self.end; 
        self.bytes_count = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory) // 
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE>  {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let size = layout.size();
        let align = layout.align();

        let addr = self.b_pos.next_multiple_of(align);
        let new_b_pos = addr + size;

        if new_b_pos > self.p_pos {
            return Err(AllocError::NoMemory);
        }

        self.b_pos = new_b_pos;
        self.bytes_count += 1;
        NonNull::new(addr as *mut u8).ok_or(AllocError::NoMemory)
    }

    fn dealloc(&mut self, _ptr: NonNull<u8>, _layout: Layout) {
        self.bytes_count -= 1;
        if self.bytes_count == 0 {
            self.b_pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE>  {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        let size = num_pages * Self::PAGE_SIZE;
        let new_p_pos = self.p_pos.checked_sub(size).ok_or(AllocError::NoMemory)?;
        let aligned_p_pos = new_p_pos & !(align_pow2 - 1);

        if aligned_p_pos < self.b_pos {
            return Err(AllocError::NoMemory);
        }

        self.p_pos = aligned_p_pos;
        Ok(self.p_pos)
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        // 
    }

    fn total_pages(&self) -> usize {
        (self.end - self.start) / Self::PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.p_pos) / Self::PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.p_pos - self.start) / Self::PAGE_SIZE
    }
}