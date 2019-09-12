use std::alloc::{GlobalAlloc, System, Layout};

pub struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout);
        panic!("yes, GlobalAlloc was installed");
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
#[used]
pub static GLOBAL: MyAllocator = MyAllocator;

