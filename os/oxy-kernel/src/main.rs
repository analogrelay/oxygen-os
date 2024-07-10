#![no_std]
#![no_main]

#![feature(error_in_core)]
#![feature(panic_info_message)]
#![feature(step_trait)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

extern crate alloc;

mod arch;
mod kernel;
pub mod memory;

pub use kernel::Kernel;

use log::error;

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
