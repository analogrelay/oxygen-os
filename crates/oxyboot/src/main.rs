#![no_main]
#![no_std]

extern crate alloc;

mod image;

use core::ptr;

use alloc::slice;
use elf::{ElfBytes, endian::AnyEndian};
use log::{info, trace};
use uefi::{prelude::*, proto::media::file::{File, FileAttribute, FileMode, FileInfo}, table::boot::{AllocateType, MemoryType}, CStr16};

use crate::image::Image;

const PAGE_SIZE: usize = 4096;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Oxygen is booting...");

    let boot_image = system_table
        .boot_services()
        .image_handle();

    let kernel_image = try_load_file(
        &mut system_table,
        boot_image,
        "efi\\boot\\oxykernel")
        .expect("Failed to load the kernel");

    // Parse the ELF headers
    let kernel = Image::try_from(kernel_image).expect("Failed to parse Kernel binary");

    info!("Kernel loaded. Entrypoint: 0x{:08X}.", kernel.elf.ehdr.e_entry);

    trace!("Exiting Boot Services");
    let (system_table, mut memory_map) = system_table.exit_boot_services(MemoryType::LOADER_DATA);

    // TODO: Set up paging

    Status::SUCCESS
}

fn try_load_file(system_table: &mut SystemTable<Boot>, image: Handle, path: &str) -> Option<&'static [u8]> {
    let mut image_file_system = system_table
        .boot_services()
        .get_image_file_system(image)
        .expect("Failed to open volume");
    let mut boot_volume = image_file_system.open_volume()
        .expect("Failed to open volume");

    let mut buf = [0u16; 256];
    if(path.len() >= 256) {
        panic!("Path is too long");
    }
    let path = CStr16::from_str_with_buf(path, &mut buf)
        .expect("Failed to convert path to UTF-16");

    let file_open_result = boot_volume
        .open(path, FileMode::Read, FileAttribute::empty());
    let mut file_handle = match file_open_result {
        Err(_) => return None,
        Ok(handle) => handle.into_regular_file().expect("Image is not a file"),
    };

    // Load file info into a small stack buffer
    let file_size = unsafe {
        let mut buf = [0u8; 500];
        let (_, buf, _) = buf.align_to_mut::<u64>();
        let mut buf = slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, buf.len() * 4);
        let file_info: &mut FileInfo = file_handle.get_info(&mut buf as &mut [u8]).expect("Failed to load file info");
        usize::try_from(file_info.file_size()).expect("File is too large")
    };

    // Allocate a buffer for the image, aligned to the NEXT frame boundary.
    let size_in_pages = ((file_size - 1) / PAGE_SIZE) + 1;
    let dest_ptr = system_table
        .boot_services()
        .allocate_pages(
            AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            size_in_pages)
        .expect("Failed to allocate space for the image") as *mut u8;

    // Zero out the buffer and reinterpret it as a slice
    let dest = unsafe {
        ptr::write_bytes(dest_ptr, 0, size_in_pages * PAGE_SIZE);
        slice::from_raw_parts_mut(dest_ptr, file_size)
    };

    // Read the image
    file_handle.read(dest).expect("Failed to read image in to memory");

    // Parse the ELF headers and return them
    let bytes = ElfBytes::<AnyEndian>::minimal_parse(dest)
        .expect("Failed to parse ELF headers");

    Some(dest)
}