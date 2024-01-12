fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: {} <kernel> <output>", args[0]);
        std::process::exit(1);
    }

    let kernel_path = std::path::Path::new(&args[1]);
    let image_path = std::path::Path::new(&args[2]);

    bootloader::UefiBoot::new(&kernel_path).create_disk_image(&image_path).unwrap();
}
