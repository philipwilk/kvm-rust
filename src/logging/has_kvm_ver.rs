use nix::{ioctl_write_int_bad, request_code_none};
use std::os::unix::prelude::RawFd;

use crate::kvm_consts::{KVM_GET_API_VER, KVM_IOCTL_ID};
use crate::logging::Severity;

// This is retrieving a NONE type ioctl... by writing nothing... what?
// Return severity | id | info
pub async fn main(kvm: RawFd) -> (Severity, String, String) {
    ioctl_write_int_bad!(
        get_kvm_api_version,
        request_code_none!(KVM_IOCTL_ID, KVM_GET_API_VER)
    );
    let kvm_api_vernum: i32 =
        unsafe { get_kvm_api_version(kvm, 0).expect("get_kvm_api_version_ERROR") };
    if kvm_api_vernum == 12 {
        (
            Severity::Info,
            "has_kvm_api".to_string(),
            "Basic kvm api found".to_string(),
        )
    } else {
        (
            Severity::Emerg,
            "has_kvm_api".to_string(),
            "Failed to get basic kvm api".to_string(),
        )
    }
}
