/*//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};
pub struct SimpleByteAllocator {
    memory_start: usize,
    memory_size: usize,
    next: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            memory_start: 0,
            memory_size: 0,
            next: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.memory_start = start;
        self.memory_size = size;
        self.next = start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        //AllocResult::Success
    }
    
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let aligned_next = (self.next + layout.align() - 1) & !(layout.align() - 1);

        // 检查是否有足够的空间分配
        if aligned_next + layout.size() <= self.memory_start + self.memory_size {
            let allocated_ptr = aligned_next;
            self.next = aligned_next + layout.size();
            AllocResult::Memory(NonZeroUsize::new(allocated_ptr).unwrap()) // 成功分配内存
        } else {
            //AllocResult::OutOfMemory // 内存不足
            AllocResult::AllocError
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        
    }

    fn total_bytes(&self) -> usize {
        self.memory_size
    }

    fn used_bytes(&self) -> usize {
        self.next - self.memory_start 
    }

    fn available_bytes(&self) -> usize {
        self.memory_start + self.memory_size - self.next
    }
}*/
use core::alloc::Layout;
use core::num::NonZeroUsize;
use crate::AllocError;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    memory_start: usize,
    memory_size: usize,
    next: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            memory_start: 0,
            memory_size: 0,
            next: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.memory_start = start;
        self.memory_size = size;
        self.next = start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult<()> {
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let aligned_next = (self.next + layout.align() - 1) & !(layout.align() - 1);

        // 检查是否有足够的空间分配
        if aligned_next + layout.size() <= self.memory_start + self.memory_size {
            let allocated_ptr = aligned_next;
            self.next = aligned_next + layout.size();
            Ok(NonZeroUsize::new(allocated_ptr).unwrap()) // 成功分配内存
        } else {
            Err(AllocError::NoMemory) // 内存不足
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        // 实现释放内存的逻辑
    }

    fn total_bytes(&self) -> usize {
        self.memory_size
    }

    fn used_bytes(&self) -> usize {
        self.next - self.memory_start 
    }

    fn available_bytes(&self) -> usize {
        self.memory_start + self.memory_size - self.next
    }
}






