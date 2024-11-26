use std::{
    env,
    process::{self, Command, ExitStatus},
};

fn main() {
    let mut qemu: Command = Command::new("qemu-system-x86_64");
    qemu.arg("-drive");
    qemu.arg(format!("format=raw,file={}", env!("BIOS_IMAGE")));
    let exit_status: ExitStatus = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}