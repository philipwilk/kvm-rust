use futures::future::join_all;
use std::os::unix::prelude::RawFd;

mod has_kvm_ver;

use crate::kvm::get_kvm_fd;
use crate::logging::has_kvm_ver::main as has_kvm_ver;

// Syslog severity levels
#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
#[allow(dead_code)] // necessary until all enums are used
pub enum Severity {
    Emerg = 0,   // System is unusable                    => A panic condition
    Alert = 1, // Action must be taken immediately        => Condition that needs immediate correction
    Crit = 2,  // Critical conditions                     => Hard device errors
    Err = 3,   // Error conditions
    Warning = 4, // Warning conditions
    Notice = 5, // Normal but significant conditions      => Conditions that are not errors but may require special handling
    Info = 6, // informational messages                   => Confirmation the program is working as expected
    Debug = 7, // Debug level messages                    => Messages that are only of use when debugging the program
}

// Filter out pfcs that are a lower severity level than chosen
pub async fn async_parse_logs_to_severity(
    logs: Vec<(Severity, String, String)>,
    level: Severity,
    modifiers: Vec<String>,
) -> Vec<(Severity, String, String)> {
    let mut notices: Vec<(Severity, String, String)> = vec![];
    for log in logs.iter() {
        /* Check if severity level is greater than notice level\
            Check that either the id is not in the excludes list, or, everything is being excluded and this id is manually included
            This may need a perfomance rework later
        */
        if log.0 <= level
            && !modifiers.contains(&("-".to_owned() + &log.1))
            && !(modifiers.contains(&("-all".to_owned()))
                && !modifiers.contains(&("+".to_owned() + &log.1)))
        {
            // does NOT remove all and not have it manually added
            notices.push(log.clone());
        }
    }
    notices
}

async fn get_startup_checks(kvm_fd: RawFd) -> Vec<(Severity, String, String)> {
    /*  Response result vector
        formatted like:
            Severity::type  => significance of condition. If expected, should be info, otherwise as appropriate
            String:         => unique id for identification by the program
            String:         => description of what happened
    */
    /*
        TODO: implement logserver spec
    */
    // Run all tests simultaneosly
    let tests = vec![has_kvm_ver(kvm_fd)];

    // Run all tests and return vector result
    join_all(tests).await
}

pub async fn get_parsed_preflights(
    _level: Severity,
    _modifiers: Vec<String>,
) -> Vec<(Severity, String, String)> {
    async_parse_logs_to_severity(get_startup_checks(get_kvm_fd()).await, _level, _modifiers).await
}

// TODO: function that acts on logs (attempt recovery/resolution)
