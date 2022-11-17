use nix::{
    ioctl_write_int_bad,
    libc::{MAP_ANONYMOUS, MAP_NORESERVE, MAP_PRIVATE, PROT_READ, PROT_WRITE},
    request_code_none, request_code_write,
    sys::mman::{mmap, MapFlags, ProtFlags},
};
use std::{
    fs::OpenOptions,
    os::unix::prelude::{IntoRawFd, RawFd},
    ptr,
};

use crate::kvm_consts::{
    KvmUserspaceMemoryRegion, KVM_CREATE_VM, KVM_IOCTL_ID, KVM_SET_USER_MEMORY_REGION,
};

/*
    create vm file descriptor, may have addition parameter for machine type, but is only relevant on S390 and arm64 platforms
*/
pub async fn create_vm(kvm_fd: RawFd, memory_size: usize) {
    ioctl_write_int_bad!(
        create_vm_fd,
        request_code_none!(KVM_IOCTL_ID, KVM_CREATE_VM)
    );
    // create the vm
    unsafe { create_vm_fd(kvm_fd, 0).expect("create_vm_fd_ERROR") };
    // assign memory to vm
    let ram_region: KvmUserspaceMemoryRegion = define_ram_region(8192, memory_size).await.unwrap();
    ioctl_write_int_bad!(
        set_memory_region,
        request_code_write!(KVM_IOCTL_ID, KVM_SET_USER_MEMORY_REGION, &|| { ram_region })
    );
}

/*
    Set aside memory region for the virtual machine
    Result -> Done, invalid data input or error
*/
pub async fn define_ram_region(
    available_capacity: usize,
    guest_capacity: usize,
) -> Result<KvmUserspaceMemoryRegion, String> {
    // check that available mem is not lower than guest memory, and positive
    if guest_capacity < available_capacity {
        return Err("Requested more memory for guest than is available on host".to_string());
    } else if guest_capacity < 1 {
        return Err(
            "Requested less memory than minimum of 1Mb: ".to_string() + &guest_capacity.to_string()
        );
    }
    // convert mebibytes to bytes
    let memory_size: usize = guest_capacity * 1048576;
    /*
        TODO: Check protflagss and mapflags implementations to correctly perform bitwise
    */
    let prot_flags: ProtFlags = PROT_READ | PROT_WRITE;
    let map_flags: MapFlags = MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE;
    let mem = unsafe { mmap(*ptr::null(), memory_size, prot_flags, map_flags, -1, 0) };

    Ok(KvmUserspaceMemoryRegion {
        slot: 0,
        flags: 0,
        guest_phys_addr: 0,
        memory_size: memory_size,
        userspace_addr: mem,
    })
}

pub async fn get_kvm_fd() -> RawFd {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .expect("Failed to open kvm")
        .into_raw_fd()
}
