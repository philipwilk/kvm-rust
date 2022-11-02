#![feature(stmt_expr_attributes)]

use futures::executor::block_on;

mod kvm_consts;
mod logging;
mod parameters;

use crate::logging::{get_parsed_preflights, Severity};
use crate::parameters::{
    get_parameters, parameters_has_key_and_its_value, parameters_to_vec_or_new, Parameters,
};

// Block initial thread and hand off to async function
fn main() {
    block_on(main_async());
}

// async main wrapper so await/async can be used
async fn main_async() {
    let parameters = get_parameters().await;

    if !parameters_has_key_and_its_value(&parameters, &Parameters::NoPreflights).unwrap() {
        let preflight_filters: Vec<String> =
            parameters_to_vec_or_new(&parameters, &Parameters::UserLogSeverityLevel);
        let preflights = get_parsed_preflights(Severity::Info, preflight_filters).await;
        if preflights.is_empty() {
            println!("No notices from pfcs to display");
        } else {
            for i in preflights {
                println!("{}, {}", i.2, i.1);
            }
        }
    }
}
