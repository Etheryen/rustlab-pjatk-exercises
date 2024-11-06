use allocators::{BumpAllocator, CustomAllocator};

mod allocators;

#[global_allocator]
static A: CustomAllocator = CustomAllocator;

fn main() {
    let a = Box::new(9);
    println!("a + a = {}", *a + *a);

    test_bump_allocator();
    test_buddy_allocator();
}

fn test_bump_allocator() {
    let mut bump_allocator = BumpAllocator::new(128);

    let block = bump_allocator.alloc(4).unwrap();
    block.fill(0xFF);

    println!("block: {:?}", block);

    let max_u32 = u32::from_be_bytes([block[0], block[1], block[2], block[3]]);

    assert_eq!(max_u32, u32::MAX);
}

fn test_buddy_allocator() {}
