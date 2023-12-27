use std::process::ExitCode;
use fgcd_parse::cli;

fn main() -> ExitCode {
    let num_processed = cli::run().unwrap();    
    println!("fgcd: compiled and processed {} characters in {} games", num_processed.1, num_processed.0);
    ExitCode::SUCCESS
}
