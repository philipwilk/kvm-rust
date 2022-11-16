use nix::{ioctl_write_int_bad, request_code_none, sys::mman::mmap, libc::{MAP_ANONYMOUS, MAP_NORESERVE, PROT_READ, PROT_WRITE, MAP_PRIVATE}};
use std::{
    fs::OpenOptions,
    os::unix::prelude::{IntoRawFd, RawFd},
};

use crate::kvm_consts::{KVM_CREATE_VM, KVM_IOCTL_ID, KvmUserspaceMemoryRegion};

/*
    create vm file descriptor, may have addition parameter for machine type, but is only relevant on S390 and arm64 platforms
*/
pub async fn create_vm(kvm_fd: RawFd) {
    ioctl_write_int_bad!(
        create_vm_fd,
        request_code_none!(KVM_IOCTL_ID, KVM_CREATE_VM)
    );
    unsafe { create_vm_fd(kvm_fd, 0).expect("create_vm_fd_ERROR") };
}

/*
    Set aside memory region for the virtual machine
    Result -> Done, invalid data input or error
*/
pub async fn define_ram_region(
    available_capacity: u64,
    guest_capacity: u64,
) -> Result<bool, String> {
    // check that available mem is not lower than guest memory, and positive
    if guest_capacity < available_capacity {
        return Err("Requested more memory for guest than is available on host".to_string());
    } else if guest_capacity < 1 {
        return Err(
            "Requested less memory than minimum of 1Mb: ".to_string() + &guest_capacity.to_string()
        );
    }
    // convert mebibytes to bytes 
    let memory_size: u64 = guest_capacity * 1048576;
    /*
        TODO: check out mmap implementation so the null value is correct
    */
    let mem = unsafe { mmap(NULL, memory_size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, -1, 0)};
    let kvm_userspace_memory_region = KvmUserspaceMemoryRegion{
        slot: 0,
        flags: 0,
        guest_phys_addr: 0,
        memory_size: memory_size,
        userspace_addr: mem,
    }
    
    Ok(true)
}

pub async fn get_kvm_fd() -> RawFd {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .expect("Failed to open kvm")
        .into_raw_fd()
}
