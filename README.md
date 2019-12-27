# temperature_converter

Convert Fahrenheit to Celsius or vice-versa.

## Description

The temperature_converter is a small command utility written in the Rust programming langaage

## Getting started

Install a recent version of Rust using your OS distributions package manager or Mozilla's own preferred rustup.  For details check with your OS distribution or visit https://rust-lang.org for more information.

### Installing

Clone the git repository from https://github.com/berrym/temperature_converter.git

### Executing program

Use Rust's own tooling to compile and run the program, e.g.

* cargo run
* cargo run ftoc 98.6
* cargo run ctof 37
* cargo run list

## Help

For help on the available commands run

* cargo run help

## Authors

Copyright 2019
Michael Berry <trismegustis@gmail.com>

## Version History
* 0.1.5
    * New function parse_command_line in mod command.
* 0.1.4
    * Moved mod command into it's own source file.
    * Added some doc tests.
* 0.1.3
    * Calculate the list of common Celsius values instead of hard coding.
    * Allow running list command as a one off from the command line.
* 0.1.2
    * Added an option to print a list of common conversions.
* 0.1.1
    * Refactored the way commands are parsed and handled.
* 0.1.0
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE file  for details.

## Acknowledgments

The excellent and freely available Rust book, for more information visit https://rust-lang.org
