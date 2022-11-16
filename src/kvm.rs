use nix::{ioctl_write_int_bad, request_code_none};
use std::os::unix::prelude::RawFd;

use crate::kvm_consts::{KVM_CREATE_VM, KVM_IOCTL_ID};

pub async fn create_vm(kvm: RawFd) {
    ioctl_write_int_bad!(
        create_vm_fd,
        request_code_none!(KVM_IOCTL_ID, KVM_CREATE_VM)
    );
    let vm_fd: i32 = unsafe { create_vm_fd(kvm, 0).expect("create_vm_fd_ERROR") };
    println!("{vm_fd}");
}
