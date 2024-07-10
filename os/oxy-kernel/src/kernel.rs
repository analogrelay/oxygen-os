use alloc::boxed::Box;
use alloc::vec::Vec;
use log::LevelFilter;
use crate::memory::KernelMemory;

/// The main entry point for the kernel.
/// The architecture-specific boot code will construct this, giving it any architecture-specific abstractions it needs.
/// Then, it will call `run` to start running the kernel.
pub struct Kernel {
    kmm: KernelMemory,
}

impl Kernel {
    pub fn new(kmm: KernelMemory) -> Self {
        Kernel {
            kmm,
        }
    }

    /// Runs the Oxygen OS Kernel.
    pub fn run(mut self) -> ! {
        log::info!("Oxygen OS Kernel started!");
        log::debug!("Debug logging enabled.");

        todo!();
    }
}
