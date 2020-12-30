use clap::{App, Arg};

use temperature_converter::temperatures;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = App::new("Temperature Converter")
        .version("0.1.8")
        .author("Michael Berry <trismegustis@gmail.com>")
        .about("Convert between Fahrenheit and Celsius")
        .arg(
            Arg::with_name("Fahrenheit to Celsius")
                .short("f")
                .long("fahrenheit-to-celsius")
                .help("Convert degree Fahrenheit to Celsius")
                .takes_value(true)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("Celsius to Fahrenheit")
                .short("c")
                .long("celsius-to-fahrenheit")
                .help("Convert degree Celsius to Fahrenheit")
                .takes_value(true)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("Print common table")
                .short("t")
                .long("table")
                .help("Print a list of common conversions"),
        )
        .get_matches();

    if cli.is_present("Fahrenheit to Celsius") {
        if let Some(n) = cli.value_of("Fahrenheit to Celsius") {
            match n.parse() {
                Ok(n) => temperatures::print_f_to_c(n),
                Err(e) => eprintln!("Error: {}", e),
            }
        };
    } else if cli.is_present("Celsius to Fahrenheit") {
        if let Some(n) = cli.value_of("Celsius to Fahrenheit") {
            match n.parse() {
                Ok(n) => temperatures::print_c_to_f(n),
                Err(e) => eprintln!("Error: {}", e),
            }
        };
    } else if cli.is_present("Print common table") {
        temperatures::print_common_table();
    } else {
        eprintln!("{}\n\nTry passing --help for more information", cli.usage());
    }
    Ok(())
}
