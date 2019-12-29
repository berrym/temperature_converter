use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

// Import command parsing attributes
mod command;
use command::{parse_command_line, Command};

// Import termperature conversion attributes
use temperature_conversion::temperatures::{
    convert, get_temperature, print_temperature, Temperature,
};

fn main() {
    let command = parse_command_line().unwrap_or_else(|e| {
        eprintln!("Could not parse command line: {}\n", e);
        usage();
        process::exit(1);
    });

    if let Err(e) = run_command(command) {
        eprintln!("\n{}\n", e);
        process::exit(1);
    };
}

// Run one of the available commands
fn run_command(command: Command) -> Result<(), &'static str> {
    match command.command.as_str() {
        "help" => Ok(usage()),
        "usage" => Ok(usage()),
        "table" => Ok(print_common_table()),
        "interactive" => Ok(run_interactive_loop()),
        "ftoc" => match command.degrees.parse() {
            Ok(t) => Ok(print_temperature(&Temperature::F(t))),
            Err(_) => {
                usage();
                Err("Error: Could not parse degrees Fahrenheit!")
            }
        },
        "ctof" => match command.degrees.parse() {
            Ok(t) => Ok(print_temperature(&Temperature::C(t))),
            Err(_) => {
                usage();
                Err("Error: Could not parse degrees Celsius!")
            }
        },
        _ => {
            usage();
            Err("Error: Unknown command!")
        }
    }
}

// Display usage information
fn usage() {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];
    match fs::read_to_string("usage.txt") {
        Ok(t) => println!("{}", t),
        Err(_) => {
            eprintln!("Valid commands\n");
            eprintln!("{} ftoc degrees_fahrenheit", prog);
            eprintln!("\tConvert given degrees Fahrenheit to Celsius\n");
            eprintln!("{} ctof degrees_celsius", prog);
            eprintln!("\tConvert given degrees Celsius to Fahrenheit\n");
            eprintln!("{} table", prog);
            eprintln!("\tPrint a list of common conversions\n");
            eprintln!("{} interactive", prog);
            eprintln!("\tRun interactive program\n");
            eprintln!("{} help or usage", prog);
            eprintln!("\tThis help message");
        }
    };
}

// Run an interactive loop of the program
fn run_interactive_loop() {
    println!("Temperature Converter\n");
    println!("Type 'exit' or 'quit' to leave at anytime\n");

    loop {
        println!("Enter a number to run a function listed below.\n");
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit");
        println!("3: Print a list of common conversions\n");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line!");

        if choice.trim() == "quit" || choice.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("\nInvalid choice!\n");
                continue;
            }
        };

        if let Err(e) = match_user_choice(choice) {
            eprintln!("{}", e);
            continue;
        };
    }
}

// Print a common list of conversions
fn print_common_table() {
    let mut celsius_table: Vec<f64> = vec![-40.0];
    for n in (-40..100).step_by(10) {
        celsius_table.push(n as f64 + 10.0);
    }

    let fahrenheit_table: Vec<f64> = celsius_table
        .iter()
        .cloned()
        .map(|x| convert(&Temperature::C(x)))
        .collect();

    for (x, y) in celsius_table
        .iter()
        .cloned()
        .zip(fahrenheit_table.iter().cloned())
    {
        println!("{:.2}ºC = {:.2}ºF", x, y);
    }
    println!();
}

// Match user's choice to appropriate function call
fn match_user_choice(choice: u8) -> Result<(), &'static str> {
    match choice {
        1 => match get_temperature("Enter ºF: ") {
            Ok(t) => Ok(print_temperature(&Temperature::F(t))),
            Err(e) => Err(e),
        },
        2 => match get_temperature("Enter ºC: ") {
            Ok(t) => Ok(print_temperature(&Temperature::C(t))),
            Err(e) => Err(e),
        },
        3 => Ok(print_common_table()),
        _ => Ok(eprintln!("\nEnter 1, 2, 3, exit, or quit!\n")),
    }
}
