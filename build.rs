use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR not set"));
    let target_dir = {
        let mut p = out_dir.clone();
        p.pop();
        p.pop();
        p.pop();
        p
    };
    let kernel_file = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_OXY_KERNEL").expect("CARGO_BIN_FILE_OXY_KERNEL not set"));

    // Copy the kernel image to a known path for use in debugging
    let kernel_path = target_dir.join("kernel.elf");
    std::fs::copy(&kernel_file, &kernel_path).expect("Failed to copy kernel image");

    let uefi_path = out_dir.join("oxy_boot.uefi.img");
    bootloader::UefiBoot::new(&kernel_file).create_disk_image(&uefi_path).expect("Failed to create UEFI disk image");

    let bios_path = out_dir.join("oxy_boot.bios.img");
    bootloader::BiosBoot::new(&kernel_file).create_disk_image(&bios_path).expect("Failed to create BIOS disk image");

    println!("cargo:rustc-env=UEFI_IMAGE_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_IMAGE_PATH={}", bios_path.display());
}
