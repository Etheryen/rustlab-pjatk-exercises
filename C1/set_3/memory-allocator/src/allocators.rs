use std::{
    alloc::{GlobalAlloc, Layout, System},
    collections::BTreeMap,
    vec,
};

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
    memory: BTreeMap<u32, Vec<u8>>,
    max_size: usize,
    curr_id: u32,
}

impl BuddyAllocator {
    pub fn new(max_size: usize) -> Self {
        Self {
            memory: BTreeMap::new(),
            max_size,
            curr_id: 0,
        }
    }

    fn size_to_allocate(size: usize) -> usize {
        let mut power = 0_usize;

        while 2_usize.pow(power.try_into().unwrap()) < size {
            power += 1;
        }

        2_usize.pow(power.try_into().unwrap())
    }

    pub fn allocated_size(&self) -> usize {
        self.memory.values().fold(0, |acc, curr| acc + curr.len())
    }

    pub fn alloc(&mut self, size: usize) -> Option<(u32, &mut [u8])> {
        if size > self.max_size {
            return None;
        }

        let memory_size = BuddyAllocator::size_to_allocate(size);

        let free_memory_size = self.max_size - self.allocated_size();

        if memory_size > free_memory_size {
            return None;
        }

        self.memory.insert(self.curr_id, vec![0_u8; memory_size]);

        self.curr_id += 1;

        Some((self.curr_id - 1, self.memory.get_mut(&(self.curr_id - 1))?))
    }

    pub fn dealloc(&mut self, id: u32) {
        self.memory
            .remove(&id)
            .expect("Deallocated unallocated memory");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bump_allocator() {
        let mut bump_allocator = BumpAllocator::new(128);

        let block = bump_allocator.alloc(4).unwrap();

        block.fill(0xFF);
    }

    #[test]
    fn test_buddy_allocator() {
        let mut buddy_allocator = BuddyAllocator::new(128);

        let (id, block) = buddy_allocator.alloc(4).unwrap();

        block.fill(0xFF);

        buddy_allocator.dealloc(id);

        assert_eq!(buddy_allocator.allocated_size(), 0);

        assert!(buddy_allocator.alloc(129).is_none());

        assert!(buddy_allocator.alloc(33).is_some());
        assert_eq!(buddy_allocator.allocated_size(), 64);

        assert!(buddy_allocator.alloc(31).is_some());
        assert_eq!(buddy_allocator.allocated_size(), 96);

        assert!(buddy_allocator.alloc(15).is_some());
        assert_eq!(buddy_allocator.allocated_size(), 112);

        assert!(buddy_allocator.alloc(15).is_some());
        assert_eq!(buddy_allocator.allocated_size(), 128);

        assert!(buddy_allocator.alloc(1).is_none());
    }
}
