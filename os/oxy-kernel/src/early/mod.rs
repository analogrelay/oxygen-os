//! Early Kernel Support Code
//! This module contains code that is used very early in the kernel's lifetime.
//! This includes the Early Logger, Early Serial Port, and Early VGA Writer devices.

use bootloader_api::{info::FrameBuffer, info::FrameBufferInfo};
use log::LevelFilter;

mod logger;
mod serial;
mod vga;

/// Initialize a text-based logger using the given pixel-based framebuffer as output.
fn init_logger(
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    log_level: LevelFilter,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    let logger = logger::LOGGER.get_or_init(move || {
        logger::LockedLogger::new(
            framebuffer,
            info,
            frame_buffer_logger_status,
            serial_logger_status,
        )
    });
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(convert_level(log_level));
}

fn convert_level(level: LevelFilter) -> log::LevelFilter {
    match level {
        LevelFilter::Off => log::LevelFilter::Off,
        LevelFilter::Error => log::LevelFilter::Error,
        LevelFilter::Warn => log::LevelFilter::Warn,
        LevelFilter::Info => log::LevelFilter::Info,
        LevelFilter::Debug => log::LevelFilter::Debug,
        LevelFilter::Trace => log::LevelFilter::Trace,
    }
}

/// Initializes early devices.
/// After this method completes, logging is available and will write to the provided framebuffer.
/// In addition, serial port logging is available.
pub fn init(fb: &'static mut FrameBuffer) {
    let info = fb.info();
    init_logger(
        fb.buffer_mut(),
        info,
        LevelFilter::Trace,
        true,
        true,
    );
}