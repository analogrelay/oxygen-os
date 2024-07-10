//! Early Kernel Support Code
//! This module contains code that is used very early in the kernel's lifetime.
//! This includes the Early Logger, Early Serial Port, and Early VGA Writer devices.

use bootloader_api::{info::FrameBuffer};

use crate::arch::x86_64::early::logger::LockedLogger;

mod logger;
mod serial;
mod vga;

/// Initialize a text-based logger using the given pixel-based framebuffer as output.
pub fn init_logger(
    framebuffer: FrameBuffer
) -> &'static LockedLogger {
    let info = framebuffer.info();
    logger::LOGGER.get_or_init(move || {
        LockedLogger::new(
            framebuffer.into_buffer(),
            info,
        )
    })
}
