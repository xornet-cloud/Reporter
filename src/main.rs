use arg_parser::ArgParser;
use core::time;
use std::thread::{self, spawn};
use ui::Ui;
use util::arcmutex;

extern crate nvml_wrapper as nvml;

mod arg_parser;
mod data_collector;
mod reporter;
mod types;
mod ui;
mod util;
use crate::reporter::Reporter;

#[tokio::main]
async fn main() {
    // Get arguments from launch
    let args = ArgParser::new();

    // Setup the terminal
    util::setup_terminal();

    // Start the reporter
    let reporter = arcmutex(Reporter::new().await.unwrap());

    if args.silent {
        println!("Xornet Reporter Started");
    }

    let data_collection_handle = spawn(move || loop {
        if !args.silent {
            let _ui = Ui::new(&args.prefix, args.no_clear, reporter.clone());
        }

        if !args.offline {
            reporter.lock().send_stats().unwrap();
        }

        thread::sleep(time::Duration::from_secs_f64(args.interval));
    });

    data_collection_handle.join().expect("main panicked");
}
