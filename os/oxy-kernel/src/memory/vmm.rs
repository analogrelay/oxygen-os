use core::ops::Range;
use crate::arch::prelude::VirtualMemoryManager;
use crate::memory::{Error, Page, PageWritability, PhysicalAddress, VirtualAddress};

#[must_use = "virtual memory changes must be flushed to take effect"]
pub trait FlushPromise {
    fn flush(self);
}

/// Provides a protocol for architecture-dependent Virtual Memory Management operations.
///
/// # About Protocols
/// Protocols are an Oxygen OS-specific concept.
/// They are similar to traits (and use traits under the hood), but are intended strictly as a compile-time assistant.
/// A protocol defines a set of methods that a type must implement, but we won't be using it as a generic constraint, nor a trait object.
/// Instead, the architecture-specific module is expected to export a struct with a known name that implements the protocol.
/// Conditional compilation will be used to select the correct implementation based on the target architecture.
pub trait VirtualMemoryManagerProtocol {
    /// The type of promise returned by `try_map` that must be flushed to apply the changes.
    type FlushPromise: FlushPromise;

    /// The size of a standard virtual memory page in bytes.
    const PAGE_SIZE: usize;

    /// A range defining the entire virtual address space.
    const VIRTUAL_ADDRESS_SPACE: Range<VirtualAddress>;

    /// A range defining the entire physical address space.
    ///
    /// NOTE: This does not necessarily mean that all physical addresses in this range are valid or usable!
    /// That depends on installed RAM, I/O memory maps, etc.
    const PHYSICAL_ADDRESS_SPACE: Range<PhysicalAddress>;

    /// A range defining the user-mode portion of the virtual address space.
    const USER_SPACE: Range<VirtualAddress>;

    /// A range defining the kernel-mode portion of the virtual address space.
    const KERNEL_SPACE: Range<VirtualAddress>;

    /// A range defining the "fixed data" (code and globals) portion of the kernel-mode virtual address space.
    const KERNEL_FIXED_SPACE: Range<VirtualAddress>;

    /// A range defining the kernel heap portion of the kernel-mode virtual address space.
    const KERNEL_HEAP_SPACE: Range<VirtualAddress>;

    /// A range defining the kernel stack portion of the kernel-mode virtual address space.
    const KERNEL_STACK_SPACE: Range<VirtualAddress>;

    /// A range defining the space used to map boot-time structures.
    /// This includes spaces for early-mode devices (VGA framebuffer, etc.) and the initial ramdrive.
    const KERNEL_BOOT_SPACE: Range<VirtualAddress>;

    /// A range defining the portion of virtual memory that is mapped to physical memory.
    ///
    /// Note that this essentially defines a maximum amount of physical memory that can be used, since the kernel
    /// will not be able to directly access physical memory that is not mapped into this range.
    const KERNEL_PHYSICAL_SPACE: Range<VirtualAddress>;

    /// Attempts to map a page to an arbitrary frame.
    fn try_map(&mut self, page: Page, writability: PageWritability) -> Result<Self::FlushPromise, Error>;

    /// Gets the physical address that represents the given virtual address, if any.
    ///
    /// Returns `None` if the virtual address is not currently mapped to a physical address.
    fn virtual_to_physical(&self, addr: VirtualAddress) -> Option<PhysicalAddress>;

    /// Gets the virtual address that can be used to read from the given physical address.
    ///
    /// Since we map the entire physical address space in to the virtual address space, this is trivial.
    fn physical_to_virtual(&self, addr: PhysicalAddress) -> VirtualAddress {
        VirtualMemoryManager::KERNEL_PHYSICAL_SPACE.start + addr.value()
    }
}
