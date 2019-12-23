use std::env;
use std::io;
use std::io::Write;
use std::process;

use temperature_conversion::command::Command;
use temperature_conversion::temperatures::{print_temperature, Temperature};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = Command::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    if let Err(e) = run_command(&args[0], command) {
        eprintln!("\n{}\n", e);
        process::exit(1);
    };
}

// Run one of the available commands
fn run_command(prog: &str, command: Command) -> Result<(), &'static str> {
    match command.command.as_str() {
        "interactive" => {
            run_interactive_loop();
            Ok(())
        }
        "ftoc" => match command.degrees.parse() {
            Ok(t) => {
                print_temperature(&Temperature::F(t));
                Ok(())
            }
            Err(_) => {
                help(prog);
                Err("Error: Could not parse degrees Fahrenheit!")
            }
        },
        "ctof" => match command.degrees.parse() {
            Ok(t) => {
                print_temperature(&Temperature::C(t));
                Ok(())
            }
            Err(_) => {
                help(prog);
                Err("Error: Could not parse degrees Celsius!")
            }
        },
        "help" => {
            help(prog);
            Ok(())
        }
        _ => {
            help(prog);
            Err("Error: Unknown command!")
        }
    }
}

// Display help
fn help(prog: &str) {
    eprintln!("Valid commands\n");
    eprintln!("{} ftoc degrees_fahrenheit", prog);
    eprintln!("\tConvert given degrees Fahrenheit to Celsius\n");
    eprintln!("{} ctof degrees_celsius", prog);
    eprintln!("\tConvert given degrees Celsius to Fahrenheit\n");
    eprintln!("{}", prog);
    eprintln!("\tRun interactive program");
}

// Run an interactive loop of the program
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

// Get a temperature from user and parse it into a usable number
fn get_temperature(prompt: &'static str) -> Result<f64, &'static str> {
    print!("{}", prompt);
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
