// KVM ioctl id
pub const KVM_IOCTL_ID: i32 = 0xAE;
// ioctl sequence numbers
pub const KVM_GET_API_VER: i32 = 0x00;
pub const KVM_CREATE_VM: i32 = 0x01;
pub const KVM_SET_USER_MEMORY_REGION: i32 = 0x46;

// Structs
#[derive(Debug)]
#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}
