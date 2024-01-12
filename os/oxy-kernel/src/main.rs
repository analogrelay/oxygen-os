#![no_std]
#![no_main]

mod early;

use bootloader_api::{BootloaderConfig, config::Mapping};
use log::info;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
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

    loop {
    }
}
