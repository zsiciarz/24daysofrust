#[macro_use]
extern crate clap;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::Drain;
use std::process;
use clap::{Arg, ArgMatches, App, SubCommand};

arg_enum! {
    #[derive(Debug)]
    enum Algorithm {
        SHA1,
        SHA256,
        Argon2
    }
}

fn run(matches: ArgMatches) -> Result<(), String> {
    let min_log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        2 | _ => slog::Level::Trace,
    };
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::FullFormat::new(decorator)
        .build()
        .filter_level(min_log_level)
        .fuse();
    let logger = slog::Logger::root(drain, o!());
    trace!(logger, "app_setup");
    // setting up app...
    debug!(logger, "load_configuration");
    trace!(logger, "app_setup_complete");
    // starting processing...
    info!(logger, "processing_started");
    match matches.subcommand() {
        ("analyse", Some(m)) => run_analyse(m, &logger),
        ("verify", Some(m)) => run_verify(m, &logger),
        _ => Ok(()),
    }
}

fn run_analyse(matches: &ArgMatches, parent_logger: &slog::Logger) -> Result<(), String> {
    let logger = parent_logger.new(o!("command" => "analyse"));
    let input = matches.value_of("input-file").unwrap();
    debug!(logger, "analysis_started"; "input_file" => input);
    Ok(())
}

fn run_verify(matches: &ArgMatches, parent_logger: &slog::Logger) -> Result<(), String> {
    let logger = parent_logger.new(o!("command" => "verify"));
    let algorithm = value_t!(matches.value_of("algorithm"), Algorithm).unwrap();
    debug!(logger, "verification_started"; "algorithm" => format!("{:?}", algorithm));
    Ok(())
}

fn main() {
    println!("24 Days of Rust vol. 2 - clap");
    let matches = App::new("24daysofrust")
        .version("0.1")
        .author("Zbigniew Siciarz")
        .about("learn you some Rust!")
        .arg(Arg::with_name("verbose").short("v").multiple(true).help(
            "verbosity level",
        ))
        .subcommand(
            SubCommand::with_name("analyse")
                .about("Analyses the data from file")
                .arg(
                    Arg::with_name("input-file")
                        .short("i")
                        .default_value("default.csv")
                        .value_name("FILE"),
                ),
        )
        .subcommand(
            SubCommand::with_name("verify")
                .about("Verifies the data")
                .arg(
                    Arg::with_name("algorithm")
                        .short("a")
                        .help("Hash algorithm to use")
                        .possible_values(&Algorithm::variants())
                        .required(true)
                        .value_name("ALGORITHM"),
                ),
        )
        .get_matches();
    if let Err(e) = run(matches) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
