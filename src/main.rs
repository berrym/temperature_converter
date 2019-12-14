use std::env;
use std::io;
use std::io::Write;
extern crate temperature_conversion;
use temperature_conversion::temperatures::{print_temperature, Temperature};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            if &args[1] == "help" {
                help(&args[0]);
            } else if &args[1] == "ftoc" {
                eprintln!("Add a degree Fahrenheit to convert to Celsius.");
            } else if &args[1] == "ctof" {
                eprintln!("Add a degree Celsius to convert to Fahrenheit.");
            } else {
                eprintln!("Error: Unknown command!\n");
                help(&args[0]);
            }
        }
        3 => run_command(args),
        _ => run_interactive_loop(),
    }
}

/// Run one off conversion commands from the command line
fn run_command(args: Vec<String>) {
    let prog = args[0].as_str();
    let command = args[1].as_str();
    let degrees = args[2].as_str();

    match command {
        "ftoc" => match degrees.parse() {
            Ok(t) => print_temperature(&Temperature::F(t)),
            Err(_) => help(prog),
        },
        "ctof" => match degrees.parse() {
            Ok(t) => print_temperature(&Temperature::C(t)),
            Err(_) => help(prog),
        },
        _ => eprintln!("Invalid command!"),
    }
}

/// Display help
fn help(prog: &str) {
    eprintln!("Valid commands\n");
    eprintln!("{} ftoc degrees_fahrenheit", prog);
    eprintln!("\tConvert given degrees Fahrenheit to Celsius\n");
    eprintln!("{} ctof degrees_celsius", prog);
    eprintln!("\tConvert given degrees Celsius to Fahrenheit\n");
    eprintln!("{}", prog);
    eprintln!("\tRun interactive program");
}

/// Run an interactive loop of the program
fn run_interactive_loop() {
    println!("Temperature Converter\n");
    println!("Type 'exit' or 'quit' to leave\n");

    loop {
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit\n");
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

        match choice {
            1 => {
                let degrees = get_temperature("Enter ºF: ");
                match degrees {
                    Ok(t) => print_temperature(&Temperature::F(t)),
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                }
            }
            2 => {
                let degrees = get_temperature("Enter ºC: ");
                match degrees {
                    Ok(t) => print_temperature(&Temperature::C(t)),
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                }
            }
            _ => eprintln!("\nEnter 1, 2, exit, or quit!\n"),
        }
    }
}

/// Get a temperature from user and parse it into a usable number
fn get_temperature(s: &'static str) -> Result<f64, &'static str> {
    print!("{}", s);
    io::stdout().flush().unwrap();

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line!");

    let temperature = match temperature.trim().parse() {
        Ok(t) => t,
        Err(_) => return Err("\nInvalid input!\n"),
    };

    Ok(temperature)
}
