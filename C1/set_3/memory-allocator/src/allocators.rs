use std::alloc::{GlobalAlloc, Layout, System};

pub struct CustomAllocator;

unsafe impl GlobalAlloc for CustomAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

pub struct BumpAllocator {
    memory: Vec<u8>,
    offset: usize,
}

impl BumpAllocator {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size],
            offset: 0,
        }
    }

    pub fn alloc(&mut self, size: usize) -> Option<&mut [u8]> {
        if self.offset + size > self.memory.len() {
            return None;
        }

        let start = self.offset;
        self.offset += size;
        Some(&mut self.memory[start..self.offset])
    }
}

pub struct BuddyAllocator {
    memory: Vec<u8>,
}
