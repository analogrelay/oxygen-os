use core::fmt::Display;
use core::iter::Step;
use crate::arch::prelude::*;
use crate::memory::{NotAlignedError, PhysicalAddress, VirtualAddress};

pub enum PageWritability {
    ReadOnly,
    ReadWrite,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Page {
    start: VirtualAddress
}

impl Display for Page {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Page({} @ {})", self.number(), self.start)
    }
}

impl Page {
    pub fn new(start: VirtualAddress) -> Result<Page, NotAlignedError> {
        if !start.is_aligned(VirtualMemoryManager::PAGE_SIZE) {
            return Err(NotAlignedError);
        }
        Ok(Page { start })
    }

    pub fn with_number(number: usize) -> Page {
        Page { start: VirtualAddress::new(number * VirtualMemoryManager::PAGE_SIZE) }
    }

    pub fn containing(address: VirtualAddress) -> Page {
        let aligned = address.align_down(VirtualMemoryManager::PAGE_SIZE);
        debug_assert!(aligned.is_aligned(VirtualMemoryManager::PAGE_SIZE));
        Page { start: aligned }
    }

    pub fn start_address(&self) -> VirtualAddress {
        self.start
    }

    pub fn number(&self) -> usize {
        self.start.value() / VirtualMemoryManager::PAGE_SIZE
    }
}

impl Step for Page {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        Step::steps_between(&start.number(), &end.number())
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        let new_number = Step::forward_checked(start.number(), count)?;
        Some(Page::with_number(new_number))
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        let new_number = Step::backward_checked(start.number(), count)?;
        Some(Page::with_number(new_number))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Frame {
    start: PhysicalAddress
}

impl Display for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Frame({} @ {})", self.number(), self.start)
    }
}

impl Frame {
    pub fn new(start: PhysicalAddress) -> Result<Frame, NotAlignedError> {
        if !start.is_aligned(VirtualMemoryManager::PAGE_SIZE) {
            return Err(NotAlignedError);
        }
        Ok(Frame { start })
    }

    pub fn containing(address: PhysicalAddress) -> Frame {
        let aligned = address.align_down(VirtualMemoryManager::PAGE_SIZE);
        debug_assert!(aligned.is_aligned(VirtualMemoryManager::PAGE_SIZE));
        Frame { start: aligned }
    }

    pub fn start_address(&self) -> PhysicalAddress {
        self.start
    }

    pub fn number(&self) -> usize {
        self.start.value() / VirtualMemoryManager::PAGE_SIZE
    }
}
