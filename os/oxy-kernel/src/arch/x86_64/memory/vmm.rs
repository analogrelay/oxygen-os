use core::ops::Range;
use bootloader_api::BootInfo;
use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, PageSize, PageTable, PhysFrame, Size4KiB, Translate};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::mapper::MapperFlush;
use crate::memory::{Error, FlushPromise, Page, PageWritability, PhysicalAddress, VirtualAddress};

pub struct VirtualMemoryManager {
    mapper: OffsetPageTable<'static>,
    frame_allocator: BootInfoFrameAllocator,
}

impl FlushPromise for MapperFlush<Size4KiB> {
    fn flush(self) {
        self.flush();
    }
}

pub const fn canonicalize_virtual_address(addr: usize) -> usize {
    if addr & 0x8000_0000_0000 == 0 {
        addr
    } else {
        addr | 0xFFFF_8000_0000_0000
    }
}

impl crate::memory::VirtualMemoryManagerProtocol for VirtualMemoryManager {
    type FlushPromise = MapperFlush<Size4KiB>;
    const PAGE_SIZE: usize = Size4KiB::SIZE as usize;

    const VIRTUAL_ADDRESS_SPACE: Range<VirtualAddress> = VirtualAddress::new(0)..VirtualAddress::new(0xFFFF_FFFF_FFFF);
    const PHYSICAL_ADDRESS_SPACE: Range<PhysicalAddress> = PhysicalAddress::new(0)..PhysicalAddress::new(0x1F_FFFF_FFFF_FFFF);
    const USER_SPACE: Range<VirtualAddress> = VirtualAddress::new(0)..VirtualAddress::new(0x8000_0000_0000);
    const KERNEL_SPACE: Range<VirtualAddress> = VirtualAddress::new(0x8000_0000_0000)..VirtualAddress::new(0xFFFF_FFFF_FFFF);

    // For simplicity, we use 16 TiB ranges for various address spaces.
    // This may change as I learn more about memory management, but for now it seems useful to have big, easily identifiable ranges.
    const KERNEL_FIXED_SPACE: Range<VirtualAddress> = VirtualAddress::new(0x8000_0000_0000)..VirtualAddress::new(0x9000_0000_0000);
    const KERNEL_HEAP_SPACE: Range<VirtualAddress> = VirtualAddress::new(0x9000_0000_0000)..VirtualAddress::new(0xA000_0000_0000);
    const KERNEL_STACK_SPACE: Range<VirtualAddress> = VirtualAddress::new(0xA000_0000_0000)..VirtualAddress::new(0xB000_0000_0000);
    const KERNEL_BOOT_SPACE: Range<VirtualAddress> = VirtualAddress::new(0xC000_0000_0000)..VirtualAddress::new(0xD000_0000_0000);
    const KERNEL_PHYSICAL_SPACE: Range<VirtualAddress> = VirtualAddress::new(0xD000_0000_0000)..VirtualAddress::new(0xFFFF_FFFF_FFFF);

    fn try_map(&mut self, page: Page, writability: PageWritability) -> Result<Self::FlushPromise, Error> {
        let frame = self.frame_allocator.allocate_frame().ok_or(Error::FrameAllocationFailed)?;
        let flags = match writability {
            PageWritability::ReadOnly => {
                use x86_64::structures::paging::PageTableFlags;
                PageTableFlags::PRESENT
            },
            PageWritability::ReadWrite => {
                use x86_64::structures::paging::PageTableFlags;
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE
            },
        };
        unsafe {
            // SAFETY: We just allocated this frame.
            Ok(self.mapper.map_to(page.into(), frame, flags, &mut self.frame_allocator)?)
        }
    }

    fn virtual_to_physical(&self, addr: VirtualAddress) -> Option<PhysicalAddress> {
        self.mapper.translate_addr(addr.into()).map(|a| a.into())
    }
}

impl VirtualMemoryManager {
    /// Initializes the memory system.
    ///
    /// # Parameters
    /// * `boot_info` - The bootloader's boot information.
    pub fn new(boot_info: &'static BootInfo) -> Self {
        let physical_memory_offset = VirtAddr::new(*boot_info.physical_memory_offset.as_ref().unwrap());
        let mapper = unsafe {
            get_page_table(physical_memory_offset)
        };
        let frame_allocator = unsafe {
            BootInfoFrameAllocator::init(&boot_info.memory_regions)
        };
        VirtualMemoryManager {
            mapper,
            frame_allocator,
        }
    }
}

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn get_page_table(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);

    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
                               -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // TODO: Doing this isn't optimal.
        // We build the iterator each time we need a new frame.
        // Ideally we'd either store the iterator (but we can't until https://github.com/rust-lang/rfcs/blob/master/text/2071-impl-trait-existential-types.md)

        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.kind == MemoryRegionKind::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions
            .map(|r| r.start..r.end);
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}
