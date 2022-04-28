pub mod temperature_conversion {
    /// Represents a valid temperature in Fahrenheit or Celsius
    pub enum Temperature {
        F(f64),
        C(f64),
    }

    // Convert Fahrenheit to Celsius
    fn farenheit_to_celsius(f: f64) -> f64 {
        (f - 32.0) * (5.0 / 9.0)
    }

    // Convert Celsius to Fahrenheit
    fn celsius_to_farenheit(c: f64) -> f64 {
        c * (9.0 / 5.0) + 32.0
    }

    /// Convert temperatures.  Take a degree Fahrenheit or Celsius
    /// and convert it to the opposite scale.
    pub fn convert(temperature: &Temperature) -> f64 {
        match *temperature {
            Temperature::F(degrees) => farenheit_to_celsius(degrees),
            Temperature::C(degrees) => celsius_to_farenheit(degrees),
        }
    }

    // Print degrees Fahrenheit to Celsius
    fn print_farenheit_to_celsius(degrees: f64) {
        println!(
            "\n{:.2}॰F = {:.2}॰C\n",
            degrees,
            convert(&Temperature::F(degrees))
        );
    }

    // Print degrees Celsius to Fahrenheit
    fn print_celsius_to_farenheit(degrees: f64) {
        println!(
            "\n{:.2}॰C = {:.2}॰F\n",
            degrees,
            convert(&Temperature::C(degrees))
        );
    }

    /// Print temperature conversions
    pub fn print_temperature_conversion(temperature: &Temperature) {
        match *temperature {
            Temperature::F(degrees) => print_farenheit_to_celsius(degrees),
            Temperature::C(degrees) => print_celsius_to_farenheit(degrees),
        }
    }

    /// Print a common table of conversions
    pub fn print_common_table() {
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
            println!("{:7.2}॰C = {:7.2}॰F", x, y);
        }
        println!();
    }
}

// Run some unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_degrees_celsius_to_farenheit() {
        assert_eq!(
            temperature_conversion::convert(&temperature_conversion::Temperature::C(0.0)),
            32.0
        );
    }

    #[test]
    fn test_100_degrees_celsius_to_farenheit() {
        assert_eq!(
            temperature_conversion::convert(&temperature_conversion::Temperature::C(100.0)),
            212.0
        );
    }

    #[test]
    fn test_32_degrees_farenheit_to_celsius() {
        assert_eq!(
            temperature_conversion::convert(&temperature_conversion::Temperature::F(32.0)),
            0.0
        );
    }

    #[test]
    fn test_212_degrees_farenheit_to_celsius() {
        assert_eq!(
            temperature_conversion::convert(&temperature_conversion::Temperature::F(212.0)),
            100.0
        );
    }

    #[test]
    fn test_98_6f_to_c() {
        assert_eq!(
            temperature_conversion::convert(&temperature_conversion::Temperature::F(98.6)),
            37.0
        );
    }

    #[test]
    fn test_negative_degrees_40_equality() {
        assert_eq!(
            temperature_conversion::convert(&temperature_conversion::Temperature::C(-40.0)),
            -40.0
        );
    }
}
