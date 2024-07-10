use acpi::AcpiTables;
use bootloader_api::BootloaderConfig;
use bootloader_api::config::Mapping;
use bootloader_api::info::Optional;
use log::LevelFilter;
use crate::arch::x86_64::{early, gdt, interrupts};
use crate::arch::x86_64::memory::VirtualMemoryManager;
use crate::Kernel;
use crate::memory::{KernelMemory, VirtualMemoryManagerProtocol};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();

    config.mappings.kernel_stack = Mapping::FixedAddress(VirtualMemoryManager::KERNEL_STACK_SPACE.start.value() as u64);
    config.mappings.physical_memory = Some(Mapping::FixedAddress(VirtualMemoryManager::KERNEL_PHYSICAL_SPACE.start.value() as u64));
    config.mappings.dynamic_range_start = Some(VirtualMemoryManager::KERNEL_BOOT_SPACE.start.value() as u64);
    config.mappings.dynamic_range_end = Some(VirtualMemoryManager::KERNEL_BOOT_SPACE.end.value() as u64);

    config
};
bootloader_api::entry_point!(entry, config = &BOOTLOADER_CONFIG);

/// Architecture specific kernel initialization, run by the bootloader.
/// Responsible for:
/// * Initializing Architecture-dependent services
/// * Preparing Architecture-dependent APIs/data structures in a way that the Kernel can use them
/// * Calling the Kernel's entry point
fn entry(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let mut fb = Optional::None;
    core::mem::swap(&mut fb, &mut boot_info.framebuffer);
    let logger = early::init_logger(fb.into_option().unwrap());
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(
        if cfg!(debug_assertions) {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    );

    // Configure interrupt and segmentation tables
    gdt::init();
    interrupts::init();

    let kmm = KernelMemory::new(VirtualMemoryManager::new(boot_info));

    Kernel::new(
        kmm,
    ).run()
}
