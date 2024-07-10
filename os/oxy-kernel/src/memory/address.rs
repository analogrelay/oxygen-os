use core::ops::{Add, Range, Sub};
use crate::arch::prelude::*;
use crate::memory::VirtualMemoryManagerProtocol;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> usize {
        self.0
    }

    pub const fn is_aligned(&self, alignment: usize) -> bool {
        self.value() % alignment == 0
    }

    pub const fn align_down(&self, alignment: usize) -> Self {
        Self::new(self.value() & !(alignment - 1))
    }

    pub const fn align_up(&self, alignment: usize) -> Self {
        Self::new((self.value() + alignment - 1) & !(alignment - 1))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    pub const fn new(value: usize) -> Self {
        let value = crate::arch::memory::canonicalize_virtual_address(value);
        Self(value)
    }

    pub const fn value(&self) -> usize {
        self.0
    }

    pub const fn is_aligned(&self, alignment: usize) -> bool {
        self.value() % alignment == 0
    }

    pub const fn align_down(&self, alignment: usize) -> Self {
        Self::new(self.value() & !(alignment - 1))
    }

    pub const fn align_up(&self, alignment: usize) -> Self {
        Self::new((self.value() + alignment - 1) & !(alignment - 1))
    }
}

macro_rules! impl_numeric_traits {
    ($t: ty, $display_suffix: literal) => {
        impl Add<usize> for $t {
            type Output = Self;

            fn add(self, rhs: usize) -> Self::Output {
                Self(self.0 + rhs)
            }
        }

        impl Sub<usize> for $t {
            type Output = Self;

            fn sub(self, rhs: usize) -> Self::Output {
                Self(self.0 - rhs)
            }
        }

        impl core::fmt::Display for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, concat!("0x{:#x}", $display_suffix), self.0)
            }
        }
    }
}

impl_numeric_traits!(PhysicalAddress, "phys");
impl_numeric_traits!(VirtualAddress, "virt");
