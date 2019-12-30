pub mod commands {
    use std::env;
    use std::fs;
    use std::io;
    use std::io::Write;

    // Import termperature conversion attributes
    use temperature_conversion::temperatures::{
        convert, get_temperature, print_temperature, Temperature,
    };

    /// Representation of a valid command
    pub struct Command {
        pub command: String,
        pub degrees: String,
    }

    impl Command {
        /// Create a new command
        ///
        /// # Arguments
        ///
        /// * `args` - A vector slice of strings that should contain
        ///            command line arguments for parsing.
        ///
        /// # Example
        /// ```
        /// let args: Vec<String> = vec![String::from("test")];
        /// commands::Command::new(&args);
        /// ```
        pub fn new(args: &[String]) -> Result<Command, &'static str> {
            let command: String;
            let degrees: String;

            // Parse command and degrees based on number of arguments given
            match args.len() {
                1 => {
                    command = "usage".to_string();
                    degrees = "".to_string();
                }
                2 => {
                    command = args[1].to_string();
                    degrees = "".to_string();
                }
                3 => {
                    command = args[1].to_string();
                    degrees = args[2].to_string();
                }
                _ => return Err("Wrong number of arguments!"),
            }

            Ok(Command { command, degrees })
        }
    }

    /// Parse the command line to create a struct Command
    pub fn parse_command_line() -> Result<Command, &'static str> {
        let args: Vec<String> = env::args().collect();
        match Command::new(&args) {
            Ok(c) => return Ok(c),
            Err(e) => return Err(e),
        }
    }

    /// Display usage information
    pub fn usage() {
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

    /// Run an interactive loop of the program
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

    /// Print a common table of conversions
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
            println!("{:7.2}ºC = {:7.2}ºF", x, y);
        }
        println!();
    }

    /// Match user's choice to appropriate function call
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

    /// Run one of the available commands
    pub fn run_command(command: Command) -> Result<(), &'static str> {
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
}

pub use commands::{parse_command_line, run_command, usage};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_command() -> Result<(), &'static str> {
        let args: Vec<String> = vec![String::from("test")];
        commands::Command::new(&args)?;
        Ok(())
    }
}
