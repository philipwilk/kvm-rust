use nix::{
    ioctl_read_bad, ioctl_write_int_bad, ioctl_write_ptr_bad,
    libc::{__u64, MAP_ANONYMOUS, MAP_NORESERVE, MAP_PRIVATE, PROT_READ, PROT_WRITE},
    request_code_none, request_code_write,
    sys::mman::{mmap, MapFlags, ProtFlags},
};
use std::{
    fs::OpenOptions,
    mem::size_of,
    os::unix::prelude::{IntoRawFd, RawFd},
    ptr::null_mut,
};

use crate::kvm_consts::{
    KvmUserspaceMemoryRegion, KVM_CREATE_VM, KVM_IOCTL_ID, KVM_SET_USER_MEMORY_REGION,
};

/*
    create vm file descriptor, may have addition parameter for machine type, but is only relevant on S390 and arm64 platforms
*/
pub async fn create_vm(kvm_fd: RawFd) {
    ioctl_write_int_bad!(
        create_vm_fd,
        request_code_none!(KVM_IOCTL_ID, KVM_CREATE_VM)
    );
    // create the vm
    unsafe { create_vm_fd(kvm_fd, 0).expect("create_vm_fd_ERROR") };
    // assign memory to vm
    let ram_region = define_ram_region(0x10000).unwrap();
    ioctl_write_ptr_bad!(
        set_memory_region,
        request_code_write!(
            KVM_IOCTL_ID,
            KVM_SET_USER_MEMORY_REGION,
            size_of::<KvmUserspaceMemoryRegion>()
        ),
        KvmUserspaceMemoryRegion
    );
    let ret: i32 =
        unsafe { set_memory_region(kvm_fd, &ram_region).expect("set_memory_region_ERROR") };
    println!("{ret}");
}

/*
    Set aside memory region for the virtual machine
    Result -> Done, invalid data input or error
*/
fn define_ram_region(capacity: usize) -> Result<KvmUserspaceMemoryRegion, String> {
    let prot_flags = ProtFlags::from_bits(PROT_READ | PROT_WRITE).unwrap();
    let map_flags = MapFlags::from_bits(MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE).unwrap();
    let mem = unsafe { mmap(null_mut(), capacity, prot_flags, map_flags, -1, 0) }.unwrap();
    Ok(KvmUserspaceMemoryRegion {
        slot: 0,
        flags: 0,
        guest_phys_addr: 0, // why does this become 0x10000 if you run an strace
        memory_size: capacity as __u64, // this also just pops off
        userspace_addr: mem as __u64, // this does not exist
    })
}

pub fn get_kvm_fd() -> RawFd {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .expect("Failed to open kvm")
        .into_raw_fd()
}
