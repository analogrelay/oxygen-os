use linked_list_allocator::LockedHeap;
use log::info;
use crate::memory;
use crate::arch::prelude::*;
use crate::memory::{Page, PageWritability, VirtualAddress, FlushPromise};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

const INITIAL_HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn initialize(vmm: &mut VirtualMemoryManager) -> Result<(), memory::Error> {
    let start_page = Page::containing(VirtualMemoryManager::KERNEL_HEAP_SPACE.start);
    let end_page = Page::containing(VirtualMemoryManager::KERNEL_HEAP_SPACE.start + INITIAL_HEAP_SIZE - 1);

    // Map the Kernel Heap
    for page in start_page..=end_page {
        vmm.try_map(page, PageWritability::ReadWrite)?.flush();
    }

    // Initialize the allocator
    unsafe {
        // SAFETY: We just mapped the heap space, so it is safe to initialize the allocator.
        ALLOCATOR.lock().init(VirtualMemoryManager::KERNEL_HEAP_SPACE.start.value() as *mut u8, INITIAL_HEAP_SIZE);
    }

    Ok(())
}
