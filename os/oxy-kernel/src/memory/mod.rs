mod page;
mod vmm;
mod kmm;
mod address;
mod error;
mod allocator;

pub use kmm::KernelMemory;
pub use error::{Error, NotAlignedError};
pub use vmm::{FlushPromise, VirtualMemoryManagerProtocol};
pub use page::{Frame, Page, PageWritability};
pub use address::{PhysicalAddress, VirtualAddress};
