use nix::{ioctl_write_int_bad, request_code_none};
use std::{
    fs::OpenOptions,
    os::unix::prelude::{IntoRawFd, RawFd},
};
// KVM ioctl id
const KVM_IOCTL_ID: i32 = 0xAE;
// ioctl sequence numbers
const KVM_GET_API_VER: i32 = 0x00;

// Syslog severity levels
#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
#[allow(dead_code)] // necessary until all enums are used
enum Severity {
    Emerg = 0,   // System is unusable                    => A panic condition
    Alert = 1, // Action must be taken immediately        => Condition that needs immediate correction
    Crit = 2,  // Critical conditions                     => Hard device errors
    Err = 3,   // Error conditions
    Warning = 4, // Warning conditions
    Notice = 5, // Normal but significant conditions      => Conditions that are not errors but may require special handling
    Info = 6, // informational messages                   => Confirmation the program is working as expected
    Debug = 7, // Debug level messages                    => Messages that are only of use when debugging the program
}

fn main() {
    // Perform pre-flights
    let pfcs = get_pre_flight_checks();
    // Get notices from pre-flights that match the chosen severity+display them
    let notices = parse_pre_flight_checks(pfcs, Severity::Info, vec![]);
    if notices.is_empty() {
        println!("No notices from pfcs to display");
    } else {
        for i in notices {
            println!("{}, {}", i.2, i.1);
        }
    }
    // TODO: function that acts on preflights (attempt recovery/resolution)
}

/* Run all checks
    Return all checks along with their severity levels and an info string
*/
fn get_pre_flight_checks() -> Vec<(Severity, String, String)> {
    /*  Response result vector
        formatted like:
            Severity::type  => significance of condition. If expected, should be info, otherwise as appropriate
            String:         => unique id for identification by the program
            String:         => description of what happened
    */
    /*
        TODO: implement logserver spec
    */
    let mut results: Vec<(Severity, String, String)> = vec![];

    // Get kvm file descriptor
    let kvm_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .expect("Failed to open kvm");
    let kvm_fd: RawFd = kvm_file.into_raw_fd();
    // Check that the kvm extension is available
    results.push(has_valid_kvm_version(kvm_fd));

    results
}

// Filter out pfcs that are a lower severity level than chosen
fn parse_pre_flight_checks(
    pfcs: Vec<(Severity, String, String)>,
    level: Severity,
    modifiers: Vec<String>,
) -> Vec<(Severity, String, String)> {
    // Checks pfcs for error types, output/remedy issues
    let mut notices: Vec<(Severity, String, String)> = vec![];
    for pfc in pfcs.iter() {
        /* Check if severity level is greater than notice
            Check that either the id is not in the excludes list, or, everything is being excluded and this id is manually included
            This may need a perfomance rework later
        */
        if pfc.0 <= level
            && !modifiers.contains(&("-".to_owned() + &pfc.1))
            && !(modifiers.contains(&("-all".to_owned()))
                && !modifiers.contains(&("+".to_owned() + &pfc.1)))
        {
            // does NOT remove all and not have it manually added
            notices.push(pfc.clone());
        }
    }
    notices
}

// This is retrieving a NONE type ioctl... by writing nothing... what?
// Return severity | id | info
fn has_valid_kvm_version(kvm: RawFd) -> (Severity, String, String) {
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
