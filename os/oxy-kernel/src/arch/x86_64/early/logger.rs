use super::{vga::FrameBufferWriter, serial::SerialPort};
use bootloader_api::info::FrameBufferInfo;
use conquer_once::spin::OnceCell;
use core::fmt::Write;
use spinning_top::Spinlock;

/// The global logger instance used for the `log` crate.
pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

/// A logger instance protected by a spinlock.
pub struct LockedLogger {
    framebuffer: Option<Spinlock<FrameBufferWriter>>,
    serial: Option<Spinlock<SerialPort>>,
}

impl LockedLogger {
    /// Create a new instance that logs to the given framebuffer.
    pub fn new(
        framebuffer: &'static mut [u8],
        info: FrameBufferInfo,
    ) -> Self {
        let framebuffer = Spinlock::new(FrameBufferWriter::new(framebuffer, info));
        let serial = Spinlock::new(unsafe { SerialPort::init() });

        LockedLogger {
            framebuffer: Some(framebuffer),
            serial: Some(serial),
        }
    }
}

impl log::Log for LockedLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if let Some(framebuffer) = &self.framebuffer {
            let mut framebuffer = framebuffer.lock();
            writeln!(framebuffer, "{:5}: {}", record.level(), record.args()).unwrap();
        }
        if let Some(serial) = &self.serial {
            let mut serial = serial.lock();
            writeln!(serial, "{:5}: {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}
