mod acpi;
pub mod memory;
mod interrupts;
mod gdt;
mod early;
mod entry;

/// The architecture-specific prelude.
/// Any code that needs to interact with the architecture-specific parts of the kernel should wildcard-import this module.
/// Note that it is essential that these re-exports are limited to those that are really needed.
pub mod prelude {
    // When re-exporting a "protocol", we should re-export the trait itself so that calling code can use it's items.
    pub use super::memory::VirtualMemoryManager;
    pub use crate::memory::VirtualMemoryManagerProtocol;
}
