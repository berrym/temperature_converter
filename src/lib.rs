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

    /// Print degrees Fahrenheit to Celsius
    pub fn print_f_to_c(degrees: f64) {
        println!(
            "\n{:.2}॰F = {:.2}॰C\n",
            degrees,
            convert(&Temperature::F(degrees))
        );
    }

    /// Print degrees Celsius to Fahrenheit
    pub fn print_c_to_f(degrees: f64) {
        println!(
            "\n{:.2}॰C = {:.2}॰F\n",
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
