use crate::arch::prelude::*;
use crate::memory::{allocator, PhysicalAddress, VirtualAddress};

/// Manages kernel memory.
///
/// Among other direct features, holding an instance of this struct guarantees that the Kernel heap
/// is initialized and ready to use.
pub struct KernelMemory {
    vmm: VirtualMemoryManager
}

impl KernelMemory {
    pub fn new(mut vmm: VirtualMemoryManager) -> Self {
        allocator::initialize(&mut vmm).expect("Failed to initialize allocator");

        KernelMemory {
            vmm
        }
    }

    pub fn vmm(&self) -> &VirtualMemoryManager {
        &self.vmm
    }
}
