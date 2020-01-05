use std::process;

// Import command parsing attributes
use temperature_converter::commands::{parse_command_line, run_command, usage};

fn main() {
    // Parse the command line to get a command
    let command = parse_command_line().unwrap_or_else(|e| {
        eprintln!("Could not parse command line: {}\n", e);
        usage();
        process::exit(1);
    });

    // Run the parsed command
    if let Err(e) = run_command(command) {
        eprintln!("\n{}\n", e);
        process::exit(1);
    };
}
