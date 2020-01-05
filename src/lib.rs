pub mod temperatures {
    /// Represents a valid temperature in Fahrenheit or Celsius
    pub enum Temperature {
        F(f64),
        C(f64),
    }

    // Convert Fahrenheit to Celsius
    fn f_to_c(f: f64) -> f64 {
        (f - 32.0) * (5.0 / 9.0)
    }

    // Convert Celsius to Fahrenheit
    fn c_to_f(c: f64) -> f64 {
        c * (9.0 / 5.0) + 32.0
    }

    /// Convert temperatures.  Take a degree Fahrenheit or Celsius
    /// and convert it to the opposite scale.
    pub fn convert(temperature: &Temperature) -> f64 {
        match *temperature {
            Temperature::F(degrees) => f_to_c(degrees),
            Temperature::C(degrees) => c_to_f(degrees),
        }
    }

    // Print degrees Fahrenheit to Celsius
    fn print_f_to_c(degrees: f64) {
        println!(
            "\n{:.2}ºF = {:.2}ºC\n",
            degrees,
            convert(&Temperature::F(degrees))
        );
    }

    // Print degrees Celsius to Fahrenheit
    fn print_c_to_f(degrees: f64) {
        println!(
            "\n{:.2}ºC = {:.2}ºF\n",
            degrees,
            convert(&Temperature::C(degrees))
        );
    }

    /// Print temperature conversions
    pub fn print_temperature(temperature: &Temperature) {
        match *temperature {
            Temperature::F(degrees) => print_f_to_c(degrees),
            Temperature::C(degrees) => print_c_to_f(degrees),
        }
    }

    pub use user_input::get_temperature;

    mod user_input {
        use std::io;
        use std::io::Write;

        /// Get a temperature from user and parse it into a usable number
        pub fn get_temperature(prompt: &'static str) -> Result<f64, &'static str> {
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
    }
}

pub mod commands {
    use std::env;
    use std::fs;
    use std::io;
    use std::io::Write;

    // Import termperature conversion attributes
    use crate::temperatures::{convert, get_temperature, print_temperature, Temperature};

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
        /// * `args` - A vector of strings that should contain
        ///            command line arguments or other user input for parsing.
        ///
        /// # Example
        ///
        /// ```
        /// # use temperature_converter::commands::Command;
        /// # fn test_command_new() -> Result<(), &'static str> {
        /// let args: Vec<String> = vec![String::from("test")];
        /// Command::new(&args)?;
        /// # Ok(())
        /// # }
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

    // Print a common table of conversions
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

// Run some unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0c_to_f() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::C(0.0)),
            32.0
        );
    }

    #[test]
    fn test_100c_to_f() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::C(100.0)),
            212.0
        );
    }

    #[test]
    fn test_32f_to_c() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::F(32.0)),
            0.0
        );
    }

    #[test]
    fn test_212f_to_c() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::F(212.0)),
            100.0
        );
    }

    #[test]
    fn test_98_6f_to_c() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::F(98.6)),
            37.0
        );
    }

    #[test]
    fn test_neg_40() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::C(-40.0)),
            -40.0
        );
    }
}
