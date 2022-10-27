use futures::executor::block_on;

mod kvm_consts;
mod logging;
use crate::logging::{get_parsed_preflights, Severity};

// Block initial thread and hand off to async function
fn main() {
    block_on(main_async());
}

// async main wrapper so await/async can be used
async fn main_async() {
    let preflight_filters: Vec<String> = vec![];
    let preflights = get_parsed_preflights(Severity::Info, preflight_filters).await;
    if preflights.is_empty() {
        println!("No notices from pfcs to display");
    } else {
        for i in preflights {
            println!("{}, {}", i.2, i.1);
        }
    }
}
