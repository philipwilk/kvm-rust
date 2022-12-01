use nix::{
    ioctl_write_int_bad, ioctl_write_ptr_bad,
    libc::{MAP_ANONYMOUS, MAP_NORESERVE, MAP_PRIVATE, PROT_READ, PROT_WRITE},
    request_code_none, request_code_write,
    sys::mman::{mlock, mmap, MapFlags, ProtFlags},
};
use std::{
    fs::OpenOptions,
    mem::size_of,
    num::NonZeroUsize,
    os::unix::prelude::{IntoRawFd, RawFd},
};

use crate::kvm_consts::{
    KvmUserspaceMemoryRegion, KVM_CREATE_VM, KVM_IOCTL_ID, KVM_SET_USER_MEMORY_REGION,
};

/*
    create vm file descriptor, may have addition parameter for machine type, but is only relevant on S390 and arm64 platforms
*/
pub async fn create_vm(kvm_fd: RawFd, capacity: usize) {
    ioctl_write_int_bad!(
        create_vm_fd,
        request_code_none!(KVM_IOCTL_ID, KVM_CREATE_VM)
    );
    // create the vm
    let vm_fd = unsafe { create_vm_fd(kvm_fd, 0).expect("create_vm_fd_ERROR") };
    // assign memory to vm
    let mem = define_ram_region(capacity).unwrap();

    ioctl_write_ptr_bad!(
        set_memory_region,
        request_code_write!(
            KVM_IOCTL_ID,
            KVM_SET_USER_MEMORY_REGION,
            size_of::<KvmUserspaceMemoryRegion>()
        ),
        KvmUserspaceMemoryRegion
    );

    let ret = unsafe { set_memory_region(vm_fd, &mem) };
    if ret.is_err() {
        println!("{mem:?}");
    }
}

/*
    Set aside memory region for the virtual machine
    Result -> Done, invalid data input or error
*/
fn define_ram_region(capacity: usize) -> Result<KvmUserspaceMemoryRegion, String> {
    let prot_flags = ProtFlags::from_bits(PROT_READ | PROT_WRITE).unwrap();
    let map_flags = MapFlags::from_bits(MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE).unwrap();
    let mem = unsafe {
        mmap(
            None,
            NonZeroUsize::new(capacity).unwrap(),
            prot_flags,
            map_flags,
            -1,
            0,
        )
    }
    .unwrap();
    let _mlocked = unsafe {
        mlock(mem, capacity).unwrap();
    };
    Ok(KvmUserspaceMemoryRegion {
        slot: 0,
        flags: 0,
        guest_phys_addr: 0,
        memory_size: capacity as u64,
        userspace_addr: mem as u64,
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
