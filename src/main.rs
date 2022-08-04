use nix::{ioctl_write_int_bad, request_code_none};
use std::{
    env::consts::ARCH,
    fs::OpenOptions,
    os::unix::prelude::{IntoRawFd, RawFd},
};
// KVM ioctl id
const KVM_IOCTL_ID: i32 = 0xAE;
// ioctl sequence numbers
const KVM_GET_API_VER: i32 = 0x00;

fn main() {
    let kvm_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .expect("Failed to open kvm");
    let kvm_fd: RawFd = kvm_file.into_raw_fd();
    println!(
        "Is kvm api compliant: {} (running on {})",
        has_valid_kvm_version(kvm_fd),
        ARCH
    );
}

// This is retrieving a NONE type ioctl... by writing nothing... what?
fn has_valid_kvm_version(kvm: RawFd) -> bool {
    ioctl_write_int_bad!(
        get_kvm_api_version,
        request_code_none!(KVM_IOCTL_ID, KVM_GET_API_VER)
    );
    let kvm_api_vernum: i32 =
        unsafe { get_kvm_api_version(kvm, 0).expect("get_kvm_api_version_ERROR") };
    println!("kvm basic api version: {}", kvm_api_vernum);
    kvm_api_vernum == 12
}
