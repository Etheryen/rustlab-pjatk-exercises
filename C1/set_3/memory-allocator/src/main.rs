use std::time::Instant;

use allocators::{BuddyAllocator, BumpAllocator, CustomAllocator};

mod allocators;

#[global_allocator]
static A: CustomAllocator = CustomAllocator;

fn main() {
    benchmark_rust_allocator();
    benchmark_bump_allocator();
    benchmark_buddy_allocator();
}

fn benchmark_rust_allocator() {
    let start = Instant::now();
    for _ in 0..2048 {
        // i32 is size 4 bytes
        let _ = Box::new(213);
    }
    let rust_time = start.elapsed();

    println!("rust_allocator time: {:?}", rust_time);
}

fn benchmark_bump_allocator() {
    let mut bump_allocator = BumpAllocator::new(8192);

    let start = Instant::now();
    for _ in 0..2048 {
        let _ = bump_allocator.alloc(4);
    }
    let bump_allocator_time = start.elapsed();

    println!("bump_allocator time: {:?}", bump_allocator_time);
}

fn benchmark_buddy_allocator() {
    let mut buddy_allocator = BuddyAllocator::new(8192);

    let start = Instant::now();
    for _ in 0..2048 {
        let _ = buddy_allocator.alloc(4);
    }
    let buddy_allocator_time = start.elapsed();

    println!("buddy_allocator time: {:?}", buddy_allocator_time);
}
