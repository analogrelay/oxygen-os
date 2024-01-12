#![no_std]
#![no_main]

#![feature(error_in_core)]
#![feature(panic_info_message)]

mod early;

use core::error;

use bootloader_api::{BootloaderConfig, config::Mapping};
use log::{info, error, debug};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        error!("PANIC! {}({}:{})", loc.file(), loc.line(), loc.column());
    } else {
        error!("PANIC! <unknown location>");
    }
    if let Some(args) = info.message() {
        error!("    {}", args);
    }
    loop {}
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};
bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    early::init(boot_info.framebuffer.as_mut().unwrap());

    info!("Booting Oxygen OS...");

    if log::log_enabled!(log::Level::Debug) {
        debug!("Kernel Image Physical Address: 0x{:016X}", boot_info.kernel_addr);
        debug!("Kernel Image Virtual Address: 0x{:016X}", boot_info.kernel_image_offset);
        debug!("RSDP Address: 0x{:016X}", boot_info.rsdp_addr.as_ref().unwrap());
        debug!("Kernel Physical Offset: 0x{:016X}", boot_info.physical_memory_offset.as_ref().unwrap());
    }

    panic!("Kernel main reached end of function");
}
