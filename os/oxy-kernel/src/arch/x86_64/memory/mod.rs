use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{PageSize, PhysFrame, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use crate::memory::{Error, Frame, PhysicalAddress, VirtualAddress};

mod vmm;

pub use vmm::{canonicalize_virtual_address, VirtualMemoryManager};

impl Into<x86_64::structures::paging::Page> for crate::memory::Page {
    fn into(self) -> x86_64::structures::paging::Page {
        x86_64::structures::paging::Page::from_start_address(self.start_address().into()).unwrap()
    }
}

impl Into<PhysicalAddress> for PhysAddr {
    fn into(self) -> PhysicalAddress {
        PhysicalAddress::new(self.as_u64() as usize)
    }
}

impl Into<PhysAddr> for PhysicalAddress {
    fn into(self) -> PhysAddr {
        PhysAddr::new_truncate(self.value() as u64)
    }
}

impl Into<VirtualAddress> for VirtAddr {
    fn into(self) -> VirtualAddress { VirtualAddress::new(self.as_u64() as usize) }
}

impl Into<VirtAddr> for VirtualAddress {
    fn into(self) -> VirtAddr { VirtAddr::new_truncate(self.value() as u64) }
}

impl<S: PageSize> Into<Frame> for PhysFrame<S> {
    fn into(self) -> Frame {
        Frame::containing(self.start_address().into())
    }
}

impl<S: PageSize> Into<PhysFrame<S>> for Frame {
    fn into(self) -> PhysFrame<S> {
        PhysFrame::containing_address(self.start_address().into())
    }
}

impl<S: PageSize> From<MapToError<S>> for Error {
    fn from(e: MapToError<S>) -> Error {
        match e {
            MapToError::PageAlreadyMapped(f) => Error::PageAlreadyMapped(f.into()),
            MapToError::FrameAllocationFailed => Error::FrameAllocationFailed,
            MapToError::ParentEntryHugePage => Error::Other("a parent entry is a huge page"),
        }
    }
}
