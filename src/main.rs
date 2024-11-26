use std::{env, fs, path::PathBuf};

fn main() {
    let current_exe: PathBuf = env::current_exe().unwrap();
    let uefi_target: PathBuf = current_exe.with_file_name("uefi.img");
    let bios_target: PathBuf = current_exe.with_file_name("bios.img");

    fs::copy(env!("UEFI_IMAGE"), &uefi_target).unwrap();
    fs::copy(env!("BIOS_IMAGE"), &bios_target).unwrap();

    println!("UEFI disk image at {}", uefi_target.display());
    println!("BIOS disk image at {}", bios_target.display());
}