// KVM ioctl id
pub const KVM_IOCTL_ID: i32 = 0xAE;
// ioctl sequence numbers
pub const KVM_GET_API_VER: i32 = 0x00;
pub const KVM_CREATE_VM: i32 = 0x01;

// Structs
#[derive(Debug)]
pub struct KvmUserspaceMemoryRegion {
    slot: u32,
    flags: u32,
    guest_phys_addr: u64,
    memory_size: u64, 
    userspace_addr: u64,
}
