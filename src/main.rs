#![allow(dead_code)]
#![allow(unused_imports)]

mod sim;
mod stats;
mod circuit;
mod algorithms;
mod cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("demo") => cli::demos::run(&args[2..]),
        Some("run") => cli::runner::run(&args[2..]),
        _           => cli::repl::run(),
    }
}
