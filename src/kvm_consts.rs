use std::ffi::c_void;
// KVM ioctl id
pub const KVM_IOCTL_ID: i32 = 0xAE;
// ioctl sequence numbers
pub const KVM_GET_API_VER: i32 = 0x00;
pub const KVM_CREATE_VM: i32 = 0x01;
pub const KVM_SET_USER_MEMORY_REGION: i32 = 0x46;

// Structs
#[derive(Debug)]
pub struct KvmUserspaceMemoryRegion {
    slot: usize,
    flags: usize,
    guest_phys_addr: usize,
    memory_size: usize,
    userspace_addr: *mut c_void,
}
