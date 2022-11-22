use nix::libc::{__u32, __u64};
// KVM ioctl id
pub const KVM_IOCTL_ID: i32 = 0xAE;
// ioctl sequence numbers
pub const KVM_GET_API_VER: i32 = 0x00;
pub const KVM_CREATE_VM: i32 = 0x01;
pub const KVM_SET_USER_MEMORY_REGION: i32 = 0x46;

// Structs
#[derive(Debug)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub memory_size: __u64,
    pub userspace_addr: __u64,
}
