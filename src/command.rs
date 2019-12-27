pub mod command {
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
        /// use temperature_conversion::command::Command;
        /// let args: Vec<String> = vec![String::from("test")];
        /// Command::new(&args);
        /// ```
        pub fn new(args: &[String]) -> Result<Command, &'static str> {
            let command: String;
            let degrees: String;

            // Parse command and degrees based on number of arguments given
            match args.len() {
                1 => {
                    command = "interactive".to_string();
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
}

pub use command::Command;
