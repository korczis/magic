extern crate log;
extern crate env_logger;

extern crate clap;
extern crate sdr_rtl;

use clap::{App, AppSettings, Arg, SubCommand};
use sdr_rtl::*;
use std::env;

const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn main() {
    let matches = App::new(DESCRIPTION)
        .settings(&[AppSettings::SubcommandRequired])
        .version(VERSION)
        .author(AUTHOR)
        .about("HackRF CLI")
        .arg(Arg::with_name("verbose")
            .help("Verbose mode")
            .short("v")
            .long("verbose")
            .multiple(true))
        .subcommand(SubCommand::with_name("info")
            .about("Show information")
            .arg(Arg::with_name("device")
                .help("Device to show info about")
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name("list")
            .about("List devices"))
        .get_matches();

    match matches.occurrences_of("verbose") {
        0 => {}
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        _ => env::set_var("RUST_LOG", "debug"),
    }

    env_logger::init().unwrap();

    if matches.is_present("info") {
        command::info::main(matches.subcommand_matches("info").unwrap());
    }

    if matches.is_present("list") {
        command::list::main();
    }
}
