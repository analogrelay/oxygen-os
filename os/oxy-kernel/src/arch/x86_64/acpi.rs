use core::ptr::NonNull;
use acpi::{AcpiError, AcpiHandler, AcpiTables, PhysicalMapping};
use log::info;
use crate::memory::{KernelMemory, PhysicalAddress, VirtualMemoryManagerProtocol};

#[derive(Clone)]
struct KernelAcpiHandler<'a>(&'a KernelMemory);

impl<'a> AcpiHandler for KernelAcpiHandler<'a> {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        // We don't need to map anything, just return the address into the physical space
        let vaddr = self.0.vmm().physical_to_virtual(PhysicalAddress::new(physical_address));
        PhysicalMapping::new(
            physical_address,
            NonNull::new(vaddr.value() as *mut T).unwrap(),
            size,
            size,
            self.clone())
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        // We don't need to do anything to unmap the region
    }
}

pub fn init(kmm: &mut KernelMemory, rsdp_address: usize) -> Result<(), AcpiError> {
    // Create a new ACPI handler to handle mapping physical addresses.
    let handler = KernelAcpiHandler(kmm);
    let tables = unsafe { AcpiTables::from_rsdp(handler, rsdp_address)? };

    info!("Dumping ACPI Platform Info");
    info!("{:#?}", tables.platform_info().unwrap());

    Ok(())
}
